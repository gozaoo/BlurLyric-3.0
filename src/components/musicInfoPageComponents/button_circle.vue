<template>
    <div class="button">
        <slot />
    </div>
</template>
<style scoped>
.button {
    border: none;
    border-radius: 50%;
    width: fit-content;
    aspect-ratio: 1/1;
    height: 1em;
    color: white;
    transition: 0.3s;
    padding: 0.5em;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
}
.button:hover {
   box-shadow: var(--Shadow-value-low);
    background-color: rgba(255, 255, 255, 0.1);
}
</style>