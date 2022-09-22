use warp::Filter;

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

/*
// 测试，打开chrome浏览器，按F12打开调试控制台，
//在Console一栏输入(或者把下面代码放入到html页面用js运行)

// 假设服务端ip为127.0.0.1
ws = new WebSocket("wss://127.0.0.1:5813/echo");
ws.onopen = function() {
    alert("连接成功");
    ws.send('幸运测试wss');
    alert("给服务端发送一个字符串：");
};
ws.onmessage = function(e) {
    alert("收到服务端的消息：" + e.data);
};
 */
pub fn echo() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let echo = warp::path("echo").and(warp::ws()).map(|ws: warp::ws::Ws| {
        log::info!("进入websockets的echo测试");
        ws.on_upgrade(|websocket| {
            use futures_util::{FutureExt, StreamExt};

            // Just echo all messages back...
            let (tx, rx) = websocket.split();
            rx.forward(tx).map(|result| {
                if let Err(e) = result {
                    eprintln!("websocket error: {:?}", e);
                }
            })
        })
    });

    echo.or(chat())
}

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

// GET /chat -> websocket upgrade
pub fn chat() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let chat = warp::path("chat")
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| {
            log::warn!("测试ws发送接收消息");
            ws.on_upgrade(|socket| user_connect(socket, users))
        });

    let chat_index = warp::path("websocket")
        .and(warp::path::end())
        .map(|| warp::reply::html(INDEX_HTML));

    chat.or(chat_index)
}

async fn user_connect(ws: WebSocket, users: Users) {
    //使用计数器为该用户分配一个新的惟一ID。
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    log::warn!("用户ID：{}", my_id);

    // 将套接字拆分为消息的发送方和接收方。
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    //使用无界通道来处理消息的缓冲和刷新到websocket…
    let (tx, rx) = mpsc::unbounded_channel::<Message>();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            log::warn!("接到什么消息就发送什么消息");
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    log::error!("websocket 发送消息出错：{}", e);
                })
                .await;
        }
    });

    //将发件人保存在连接用户列表中。
    users.write().await.insert(my_id, tx);

    //返回一个Future，它基本上是一个状态机
    //这个特定的用户连接。

    //每当用户发送消息时，将其广播到
    //所有其他用户…
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("websocket 出错(uid={}): {}", my_id, e);
                break;
            }
        };
        log::info!("websocket广播发送消息:{:?}", msg);
        user_message(my_id, msg, &users).await;
    }
}

async fn user_message(my_id: usize, msg: Message, users: &Users) {
    // 跳过任何非文本信息…
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        println!("here");
        return;
    };
    let new_msg = format!("<用户#{}>: {}", my_id, msg);
    log::warn!("消息：{}", new_msg);

    // 来自此用户的新消息，将其发送给其他所有人(除了相同的uid)…
    for (&uid, tx) in users.read().await.iter() {
        if my_id != uid {
            if let Err(disconnected) = tx.send(Message::text(new_msg.clone())) {
                //处理发送消息失败
                log::error!("发送消息失败:{}", disconnected);
            }
        }
    }
}

static INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Warp Chat</title>
    </head>
    <body>
        <h1>Warp chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">Send</button>
        <script type="text/javascript">
        const chat = document.getElementById('chat');
        const text = document.getElementById('text');
        const uri = 'wss://' + location.host + '/chat';
        const ws = new WebSocket(uri);

        function message(data) {
            const line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }

        ws.onopen = function() {
            chat.innerHTML = '<p><em>Connected!</em></p>';
        };

        ws.onmessage = function(msg) {
            message(msg.data);
        };

        ws.onclose = function() {
            chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
        };

        send.onclick = function() {
            const msg = text.value;
            ws.send(msg);
            text.value = '';

            message('<You>: ' + msg);
        };
        </script>
    </body>
</html>
"#;
