<script>
    import "../app.css";
</script>

<header>
    <nav class="flex justify-between p-0 m-0 items-baseline">
        <h1 class="title text-3xl p-0 m-0">Clean hotel</h1>
        <a href="/">Home</a>
        <a href="/register">Register</a>
        <a href="/login">Login</a>
    </nav>
</header>

<slot />

<style lang="postcss">
    nav {
        column-gap: 2ch;
    }

    .title {
        flex-grow: 2;
    }

    a {
        padding: 0;
        margin: 0;
        text-decoration: none;
        font-size: 1.2rem;
        /* color: white;
        background-color: #0066ff; */
    }

    a:hover {
        text-decoration: underline;
    }
</style>
