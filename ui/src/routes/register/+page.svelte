<main class="w-96">
    <form action="/register" method="post" class="m-auto ">
        <div>
            <label for="email" class="block mb-2 text-sm font-medium"
                >Email:</label
            >
            <input
                type="email"
                id="email"
                name="email"
                class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                required
            />
        </div>
        <div>
            <label for="confirm_password" class="block mb-2 text-sm font-medium"
                >Password:</label
            >
            <input
                type="password"
                id="password"
                name="_password"
                minlength="8"
                class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                required
            />
        </div>
        <div>
            <label for="confirm_password" class="block mb-2 text-sm font-medium"
                >Confirm Password:</label
            >
            <input
                type="password"
                id="confirm_password"
                name="confirm_password"
                class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                required
            />
        </div>
        <input
            type="submit"
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold rounded py-2 px-4"
            value="Sign up"
        />
        <p class="text-sm text-gray-500 dark:text-gra-400">
            Already have an account? <a
                href="/login"
                class="font-medium text-primary-600 hover:underline dark:text-primary:500 text-blue-400"
                >Login here</a
            >
        </p>
    </form>
</main>
