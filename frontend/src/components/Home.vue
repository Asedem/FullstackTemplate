<script setup>
import { ref, onMounted } from 'vue';

const users = ref([]);
const isLoading = ref(true);
const error = ref(null);

const fetchUsers = async () => {
    try {
        const response = await fetch('http://localhost:8080/users');
        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
        }
        const data = await response.json();
        users.value = data;
    } catch (e) {
        error.value = `Failed to fetch users: ${e.message}`;
        console.error("Error fetching users:", e);
    } finally {
        isLoading.value = false;
    }
};

onMounted(() => {
    fetchUsers();
});
</script>

<template>
    <div class="container">
        <div class="card">
            <h1 class="title">
                Users List
            </h1>

            <div v-if="isLoading" class="message loading-message">
                Loading users...
            </div>

            <div v-else-if="error" class="message error-message">
                {{ error }}
            </div>

            <div v-else-if="users.length === 0" class="message no-users-message">
                No users found.
            </div>

            <div v-else class="user-grid">
                <div v-for="user in users" :key="user.id" class="user-card">
                    <div class="user-avatar-wrapper">
                        <div class="user-avatar">
                            {{ user.user_name.charAt(0).toUpperCase() }}
                        </div>
                        <div>
                            <p class="user-name">
                                {{ user.user_name }}
                            </p>
                            <p class="user-id">ID: {{ user.id }}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
body {
    margin: 0;
    font-family: 'Inter', sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
}

.container {
    min-height: 100vh;
    background-color: #f3f4f6;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
}

.card {
    background-color: #ffffff;
    padding: 2rem;
    border-radius: 0.75rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
    width: 100%;
    max-width: 42rem;
}

.title {
    font-size: 2.25rem;
    font-weight: 800;
    text-align: center;
    color: #1f2937;
    margin-bottom: 2rem;
    letter-spacing: -0.025em;
}

.message {
    text-align: center;
    font-size: 1.125rem;
}

.loading-message {
    color: #2563eb;
}

.error-message {
    color: #dc2626;
}

.no-users-message {
    color: #4b5563;
}

.user-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1.5rem;
}

@media (min-width: 768px) {
    .user-grid {
        grid-template-columns: repeat(2, 1fr);
    }
}

.user-card {
    background-color: #eff6ff;
    padding: 1.5rem;
    border-radius: 0.5rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    transition: box-shadow 0.3s ease-in-out;
    border: 1px solid #bfdbfe;
}

.user-card:hover {
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

.user-avatar-wrapper {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.user-avatar {
    flex-shrink: 0;
    background-color: #3b82f6;
    color: #ffffff;
    border-radius: 9999px;
    height: 3rem;
    width: 3rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    font-weight: 700;
}

.user-name {
    font-size: 1.25rem;
    font-weight: 600;
    color: #1f2937;
    margin: 0;
}

.user-id {
    font-size: 0.875rem;
    color: #4b5563;
    margin: 0;
}
</style>
