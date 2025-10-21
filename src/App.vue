<template>
  <h1>Hello App!</h1>
  <p>
    <strong>Current route path:</strong> {{ $route.fullPath }}
  </p>
  <nav>
    <RouterLink to="/">Go to Home</RouterLink>
    <RouterLink to="/about">Go to About</RouterLink>
    <RouterLink to="/terminal">Go to Terminal</RouterLink>
  </nav>
  <main>
    <RouterView />
  </main>
  <footer class="bg-red-100 text-center p-4 text-sm text-gray-500">
      © 2025 Il tuo nome
    </footer>
</template>