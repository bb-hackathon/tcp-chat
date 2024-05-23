const createChat = document.getElementById('create_room');
createChat.addEventListener('click', () => {
    window.location.href = "create_room.html";
});

function sendMessage() {
    var messageInput = document.getElementById("messageInput");
    var message = messageInput.value;
    if (message.trim() !== "") {
        fetch("http://localhost:8080/send", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ message: message })
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

window.onload = function() {
    var scrollContainer = document.getElementById('chat');
    scrollContainer.scrollTop = scrollContainer.scrollHeight;
};

document.addEventListener("DOMContentLoaded", () => {
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
            div.addEventListener('click', async () =>  {
                const response = await fetch(`https://localhost:8080/spitmessages?room=${li.dataset.chatId}`);
                const chat = document.getElementById('chat')
                response.forEach(element => {
                    const chatli = chat.createElement('li')
                    chatli.classList.add('you')
                    const message = chat.createElement('div')
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

    fetchChatList();
});
