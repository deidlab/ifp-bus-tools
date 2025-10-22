<template>
  <!--  BEGIN MAIN CONTAINER  -->
  <!-- <div class="relative">
    <div class="main-container text-black dark:text-white-dark min-h-screen">
      <div class="main-content flex flex-col min-h-screen p-2">
        <Header />
        <main>
          <RouterView />
        </main>
        <footer class="bg-red-100 text-center p-4 text-sm text-gray-500">
          © 2025 Il tuo nome
        </footer>
        <Footer />
      </div>
    </div>
  </div> -->
  <div class="flex flex-col min-h-screen">
    <!-- Header -->
    <header class="bg-blue-600 text-white p-4">
      <h1 class="text-lg font-bold">My App Header</h1>
    </header>

    <!-- Main Layout -->
    <div class="flex flex-1">
      <!-- Sidebar -->
      <aside class="hidden md:block w-64 bg-gray-100 border-r p-4">
        <nav>
          <ul class="space-y-2">
            <li><a href="#" class="block hover:text-blue-600">Dashboard</a></li>
            <li>
              <a href="#" class="block hover:text-blue-600">Impostazioni</a>
            </li>
          </ul>
        </nav>
      </aside>

      <!-- Main Content -->
      <main class="flex-1 p-4 bg-white">
        <router-view />
      </main>
    </div>

    <!-- Footer -->
    <footer class="bg-gray-200 text-center p-4">
      <p>© 2025 - My App</p>
    </footer>
  </div>
</template>

<script setup lang="ts">
import Header from "./components/layout/Header.vue";
import Footer from "./components/layout/Footer.vue";
</script>
