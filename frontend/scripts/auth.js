const signUpButton = document.getElementById('signUp');
const signInButton = document.getElementById('signIn');
const container = document.getElementById('container');
const registerButton = document.getElementById('register');
const loginButton = document.getElementById('login');
signUpButton.addEventListener('click', () => {
    container.classList.add("right-panel-active");
});

signInButton.addEventListener('click', () => {
    container.classList.remove("right-panel-active");
});

registerButton.addEventListener('click', () => {
    var loginInput = document.getElementById("register_login");
    var login = loginInput.value;
    var passwordInput = document.getElementById("register_password");
    var password = passwordInput.value;

    if (login.trim() !== "" && password.trim() !== "") {
        fetch("http://localhost:8080/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ login: login, password: password })
        })
        .then(response => response.json())
        .then(data => {
            console.log("Message sent:", data);
            loginInput.value = "";
            passwordInput.value = "";
            window.location.href = "index.html";

        })
        .catch(error => {
            console.error("Error sending message:", error);
        });
    }
});

loginButton.addEventListener('click', () => {
    var loginInput = document.getElementById("login_login");
    var login = loginInput.value;
    var passwordInput = document.getElementById("login_password");
    var password = passwordInput.value;

    if (login.trim() !== "" && password.trim() !== "") {
        fetch("http://localhost:8080/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ login: login, password: password })
        })
        .then(response => response.json())
        .then(data => {
            console.log("Message sent:", data);
            loginInput.value = "";
            passwordInput.value = ""; 
            window.location.href = "index.html";
        })
        .catch(error => {
            console.error("Error sending message:", error);
        });
    }
});

