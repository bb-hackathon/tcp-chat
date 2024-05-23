function showAddButton(input) {
    const addButton = input.nextElementSibling;
    if (input.value.trim() !== "") {
        addButton.style.display = "inline-block";
    } else {
        addButton.style.display = "none";
    }
}

function addUserInput(button) {
    const userInputDiv = document.createElement('div');
    userInputDiv.classList.add('user-input');
    userInputDiv.innerHTML = `
        <input type="text" placeholder="Введите имя пользователя" oninput="showAddButton(this)">
        <button class="add-button" style="display: none;" onclick="addUserInput(this)">+</button>
    `;
    document.getElementById('user-list').appendChild(userInputDiv);
    button.style.display = "none";
}

function createChat() {
    const roomNameInput = document.getElementById('room-name-input');
    const userInputs = document.querySelectorAll('.user-input input');
    const users = [];
    userInputs.forEach(input => {
        if (input.value.trim() !== "") {
            users.push(input.value.trim());
        }
    });
    fetch("http://localhost:8080/createroom", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({room: roomNameInput, usernames: users})
        })
        .then(data => {
            alert("Чат создан с пользователями: " + users.join(', '));
            console.log("Чат создан:", data);
            window.location.href = "index.html";
        })
        .catch(error => {
            console.error("Ошибка при создании чата:", error);
        });
}
