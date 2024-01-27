<!-- DefaultLayout.vue with named slots -->
<template>
    <div>
        <div class="ui top attached button">
            <slot name="nav-top"></slot>
        </div>

        <div class="ui grid">
            <div class="three wide column">
                <slot name="nav-side"></slot>
            </div>
            <div class="thirteen wide column">
                <slot name="main"></slot>
            </div>
        </div>
    </div>
</template>
