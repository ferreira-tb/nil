<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Chat from '@/components/chat/Chat.vue';
import { useToggle, whenever } from '@vueuse/core';
import ChatIcon from '@/components/chat/ChatIcon.vue';
import { Popover, PopoverContent, PopoverTrigger } from '@tb-dev/vue-components';

const { player } = NIL.player.refs();

const [isChatOpen, toggleChat] = useToggle(false);
const closeChat = () => void toggleChat(false);

const [hasUnread, toggleUnread] = useToggle(false);

whenever(isChatOpen, () => void toggleUnread(false));

function onChatUpdated({ message }: ChatUpdatedPayload) {
  if (!isChatOpen.value && message.author.id !== player.value?.id) {
    hasUnread.value = true;
  }
}
</script>

<template>
  <Popover v-model:open="isChatOpen">
    <PopoverTrigger as-child>
      <ChatIcon :has-unread />
    </PopoverTrigger>

    <PopoverContent
      align="end"
      :align-offset="-15"
      side="top"
      :side-offset="10"
      class="w-96 max-w-[90vw] h-[500px] max-h-[75vh]"
      @pointer-down-outside="closeChat"
    >
      <Chat
        class="w-full h-[470px] max-h-[calc(75vh-30px)]"
        @chat-updated="onChatUpdated"
      />
    </PopoverContent>
  </Popover>
</template>
