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
            const response = await fetch('https://example.com/api/chat-list');
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
            li.appendChild(div);
            chatListElement.appendChild(li);
        });
    }

    fetchChatList();
});
