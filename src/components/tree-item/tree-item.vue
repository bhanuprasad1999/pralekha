<template>
    <div>
        <div class="item" v-if="!isFolder">
            <i class="file icon"></i>
            <div class="content" @click.stop="fileSelectedEvent">
                <div class="header">{{ item.name }}</div>
                <div class="description">{{ item.description }}</div>
            </div>
        </div>
        <div class="item" v-else>
            <i class="folder icon"></i>
            <div class="content" @click.stop="toggle">
                <div class="header">{{ item.name }}</div>
                <div class="description">{{ item.description }}</div>
            </div>
            <div class="list" v-show="isOpen">
                <tree-item class="item" v-for="(child, index) in item.children" :key="index" :item="child"
                    @file-selected="relativePathFile"></tree-item>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, defineProps, defineEmits } from 'vue';

const props = defineProps({
    item: Object,
});


const isOpen = ref(false);
const emit = defineEmits(['file-selected']);

const isFolder = computed(() => {
    return props.item.children && props.item.children.length;
});

const toggle = (event) => {
    if (isFolder.value) {
        isOpen.value = !isOpen.value;
    }
};

const fileSelectedEvent = () => {
    emit('file-selected', props.item)
};

const relativePathFile = (childItem) => {
    emit('file-selected', childItem); // Emit the received item
};
</script>

<style scoped>
/* Add styles if necessary */
</style>
