<!-- HomePage.vue with named slots -->
<template>
    <DefaultLayout>
        <template v-slot:nav-top>
            <h1>@Cosquntime</h1>
        </template>
        <template v-slot:nav-side>
            <tree-item class='ui list' :item="fileStructure" @file-selected="fileSelected" />
        </template>
        <template v-slot:main>
            <preview></preview>
        </template>
    </DefaultLayout>
</template>

<script setup>
import DefaultLayout from '../layout/default/Default.vue';

import TreeItem from '@/components/tree-item/tree-item.vue';
import buildJsonTree from '@/components/tree-item/utils.js';
import preview from '@/components/editor/preview.vue';

import { ref, onMounted } from 'vue';

// tauri imports
import { invoke } from '@tauri-apps/api/tauri';


const fileStructure = ref({});
let fileDetails = ref({});



const fileSelected = (item) => {
    console.log('Event received in parent:', item);
    fileDetails.value = item;
};

onMounted( async () => {
    try {
        const details = await invoke('read_file_details', { path: "/home/proq/bhanuprasad-drive/personal-site/tmp" });
        fileStructure.value = buildJsonTree(details);
    } catch (e) {
        console.log("Error", e);
    }
});

</script>
