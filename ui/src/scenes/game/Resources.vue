<!-- Copyright (C) Tsukilabs contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
const { resources } = NIL.player.refs();
</script>

<template>
  <div class="grid grid-cols-4 items-center gap-8">
    <Wood :amount="resources?.wood" />
    <Stone :amount="resources?.stone" />
    <Iron :amount="resources?.iron" />
    <Food :amount="resources?.food" />
  </div>
</template>
