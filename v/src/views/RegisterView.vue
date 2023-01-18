<template>
    <form action="views">
        <div class="container">
            <h1>Register</h1>
            <label for="email">Email</label>
            <input type="text" placeholder="Enter email" name="email" id="email" required>

            <label for="password">Password</label>
            <input type="password" placeholder="Enter password" name="pswd" id="password" required>

            <label for="password">Confirm password</label>
            <input type="password" placeholder="Confirm password" name="pswd-confirm" id="password-confirm" required>
        <button type="submit" class="registerbtn">Register</button>
        <a href="#">Already have an account? Try loggin in</a>
        </div>
    </form>
</template>

<style scoped>
input[type=text], input[type=password] {
    display: inline-block;
    padding: 10px;
    margin-bottom: 10px;
    width: 100%;
    background-color: #444;
    border: none;
    color: #ddd;
}

form {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, 50%);
}

.registerbtn {
    background-color: #04AA6D;
    color: white;
    width: 100%;
    padding:16px 0;
    margin: 10px 0;
    border: none;
}
</style>
