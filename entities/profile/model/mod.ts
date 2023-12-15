import { createEvent } from 'effector'
import { createStore } from 'effector'
import { persist }     from 'effector-storage/local'

export const $nickname = createStore('Player', { name: 'nickname' })

export const setNickname = createEvent<string>()

$nickname.on(setNickname, (_, newNickname) => newNickname)

persist({ store: $nickname })
