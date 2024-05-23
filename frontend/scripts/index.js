const createChat = document.getElementById('create_room');
createChat.addEventListener('click', () => {
    window.location.href = "create_room.html";
});
var active_room = '';
function sendMessage() {
    var messageInput = document.getElementById("messageInput");
    var message = messageInput.value;
    if (message.trim() !== "") {
        fetch("http://localhost:8080/send", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ message: message, UUID: active_room})
        })
        .then(response => response.json())
        .then(data => {
            console.log("Message sent:", data);
            messageInput.value = ""; 
        })
        .catch(error => {
            console.error("Error sending message:", error);
        });
    }
}
let eventSource;
window.onload = function() {
    var scrollContainer = document.getElementById('chat');
    scrollContainer.scrollTop = scrollContainer.scrollHeight;
};

document.addEventListener("DOMContentLoaded", async () => {await updateGroups()})

async function updateGroups(){
    async function fetchChatList() {
        try {
            const response = await fetch('http://localhost:8080/spitroom');
            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status}`);
            }
            const chatList = await response.json();
            updateChatList(chatList);
        } catch (error) {
            console.error('Error fetching chat list:', error);
        }
    }

    function updateChatList(chats) {
        const chatListElement = document.getElementById('chat-list');
        chatListElement.innerHTML = '';

        chats.forEach(chat => {
            const li = document.createElement('li');
            li.dataset.chatId = chat.id; 

            const div = document.createElement('div');
            const p = document.createElement('p');
            p.textContent = chat.name;
            const divSeparator = document.createElement('div');
            divSeparator.classList.add('chat-name-sep');

            div.appendChild(p);
            div.appendChild(divSeparator);
            div.addEventListener('click', async() =>  {
                active_room = li.dataset.chatId;
                var chat_name = document.getElementById("room-name")
                subscribeToRoom(active_room)
                chat_name.textContent = p.textContent;
                const response = await fetch(`http://localhost:8080/spitmessages?room_id=${li.dataset.chatId}`);
                console.log(response)
                const response2 = await response.json()
                const chat = document.getElementById('chat')
                chat.replaceChildren()
                response2.forEach(element => {
                    const chatli = document.createElement('li')
                    chatli.classList.add('you')
                    const message = document.createElement('div')
                    message.classList.add('message')
                    message.innerHTML += element
                    chatli.appendChild(message)
                    chat.appendChild(chatli)
                });
            }); 
            li.appendChild(div);
            chatListElement.appendChild(li);
        });
    }

    await fetchChatList();
};

async function updateMessages(){
    const response = await fetch(`http://localhost:8080/spitmessages?room_id=${active_room}`);
    console.log(response)
    const response2 = await response.json()
    const chat = document.getElementById('chat')
    chat.replaceChildren()
    response2.forEach(element => {
        const chatli = document.createElement('li')
        chatli.classList.add('you')
        const message = document.createElement('div')
        message.classList.add('message')
        message.innerHTML += element
        chatli.appendChild(message)
        chat.appendChild(chatli)
    });
}

function subscribeToRoom() {
    if (eventSource) {
        eventSource.close();
    }

    eventSource = new EventSource(`http://localhost:8080/streammessages?room_id=${active_room}`);
    eventSource.onmessage = function(event) {
        const message = JSON.parse(event.data);
        const chatli = document.createElement('li')
        chatli.classList.add('you')
        const messagediv = document.createElement('div')
        messagediv.classList.add('message')
        messagediv.innerHTML += element
        chatli.appendChild(message)
        chat.appendChild(chatli)
    };

    eventSource.onerror = function(event) {
        console.error("Ошибка соединения с сервером:", event);
        eventSource.close();
    };
}

setInterval(async () => {
    await updateGroups()
}, 3000);