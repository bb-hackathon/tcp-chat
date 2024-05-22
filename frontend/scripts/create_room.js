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
    const userInputs = document.querySelectorAll('.user-input input');
    const users = [];
    userInputs.forEach(input => {
        if (input.value.trim() !== "") {
            users.push(input.value.trim());
        }
    });
    alert("Чат создан с пользователями: " + users.join(', '));
}
