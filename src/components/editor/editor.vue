<template>
    <button @click="updateMain">Edit</button>
</template>

<script setup>
import { defineEmits } from 'vue';
const emit = defineEmits(['update-main']);

const updateMain = ()=>{
    emit('update-main', 'helloo world')
}
</script>