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