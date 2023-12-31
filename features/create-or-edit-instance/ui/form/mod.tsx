/* eslint-disable react/jsx-no-useless-fragment, react/jsx-fragments */

import      { Fragment }    from 'react'
import      { useCallback } from 'react'
import      { useEffect }   from 'react'
import      { useMemo }     from 'react'

import      { zodResolver } from '@hookform/resolvers/zod'
import      { Button }      from '@mantine/core'
import      { Checkbox }    from '@mantine/core'
import      { Divider }     from '@mantine/core'
import      { Drawer }      from '@mantine/core'
import      { Flex }        from '@mantine/core'
import      { Input }       from '@mantine/core'
import      { NumberInput } from '@mantine/core'
import      { rem }         from '@mantine/core'
import      { Select }      from '@mantine/core'
import      { Skeleton }    from '@mantine/core'
import      { Slider }      from '@mantine/core'
import      { Stack }       from '@mantine/core'
import      { IconX }       from '@tabler/icons-react'
import      { join }        from '@tauri-apps/api/path'
import      { nanoid }      from 'nanoid/non-secure'
import      { Controller }  from 'react-hook-form'
import      { useForm }     from 'react-hook-form'
import      { z }           from 'zod'

import type { Instance }    from '@entity/instance'
import      { add }         from '@entity/instance'
import      { update }      from '@entity/instance'
import      { $settings }   from '@entity/settings'
import      { useLookup }   from '@entity/version'

export const schema = z.object({
	name: z.string().min(3).max(40),
	path: z.string(),
	width: z.number().min(800).max(3840),
	height: z.number().min(600).max(2160),
	fullscreen: z.boolean(),
	extraArgs: z.string(),
	alloc: z.number().min(512).max(16384),
	version: z.string(),
})

type Schema = z.infer<typeof schema>

interface Props {
	edit?: Instance
	opened?: boolean
	onClose?: () => void
}

const defaultValues: Partial<Schema> = {
	name: '',
	path: '',
	width: 1280,
	height: 720,
	fullscreen: false,
	extraArgs: '',
	alloc: 2048,
}

export function Form(props: Props) {
	// eslint-disable-next-line @typescript-eslint/no-empty-function
	const { edit, opened = false, onClose = () => {} } = props

	const { register, formState, handleSubmit, control, watch, reset, setValue } = useForm<Schema>({
		defaultValues,
		reValidateMode: 'onChange',
		resolver: zodResolver(schema),
	})

	const path = watch('path')

	useEffect(() => {
		if (!path) {
			void (async () => {
				setValue('path', await join($settings.getState().rootPath, 'instances', nanoid(6)))
			})()
		}
	}, [path, setValue])

	useEffect(() => {
		if (edit) {
			reset({
				...edit,
				version: `${edit.version.provider}-${edit.version.vid}`,
			})
		} else {
			reset(defaultValues)
		}
	}, [edit, reset])

	const close = useCallback(() => {
		onClose()
	}, [onClose])

	const { data: versions, status, error } = useLookup()

	const submit = useCallback(
		(data: Schema) => {
			if (edit) {
				update({
					id: edit.id,
					payload: {
						...data,
						version: Object.values(versions ?? [])
							.flat()
							.find((it) => `${it.provider}-${it.vid}` === data.version)!,
					},
				})
			} else {
				add({
					...data,
					version: Object.values(versions ?? [])
						.flat()
						.find((it) => `${it.provider}-${it.vid}` === data.version)!,
				})
			}

			onClose()
		},
		[edit, onClose, versions],
	)

	const versionSelectOptions = useMemo(
		() =>
			versions &&
			Object.entries(versions).map(([group, items]) => ({
				group,
				items: items.map(({ vid, provider }) => ({ label: vid, value: `${provider}-${vid}` })),
			})),
		[versions],
	)

	return (
		<Drawer opened={opened} onClose={close} position='right'>
			<Flex
				component='form'
				onSubmit={handleSubmit(submit)}
				direction='column'
				justify='space-between'
				style={{ height: '100%' }}
			>
				<Stack gap={8}>
					<Input.Wrapper label='Name' error={formState.errors.name?.message}>
						<Input {...register('name')} error={Boolean(formState.errors.name)} />
					</Input.Wrapper>
					<Input.Wrapper
						label='Instance path'
						description='Path to save files and mods'
						error={formState.errors.path?.message}
					>
						<Input variant='filled' disabled {...register('path')} error={Boolean(formState.errors.path)} />
					</Input.Wrapper>
					<Divider />
					<Skeleton visible={status === 'pending'}>
						<Input.Wrapper
							label='Assigned version'
							description='This is the version that the instance will run with. If the version you need is not listed here, install it.'
							error={formState.errors.version?.message ?? error?.message}
						>
							<Controller
								control={control}
								render={({ field }) => (
									<Select
										placeholder='Select version'
										allowDeselect={false}
										searchable
										data={versionSelectOptions ?? []}
										error={status === 'error' || Boolean(formState.errors.version)}
										m={5}
										onChange={field.onChange}
										value={field.value}
										onBlur={field.onBlur}
									/>
								)}
								name='version'
							/>
						</Input.Wrapper>
					</Skeleton>
					<Divider />
					<Flex align='center' justify='space-between'>
						<Flex gap={8} align='center'>
							<Controller
								control={control}
								render={({ field }) => (
									<NumberInput
										rightSection={<Fragment />}
										rightSectionWidth={0}
										style={{ width: rem(56) }}
										onBlur={field.onBlur}
										value={field.value}
										onChange={(it) => field.onChange({ target: { value: it } })}
										error={Boolean(formState.errors.width)}
									/>
								)}
								name='width'
							/>
							<IconX size={20} />
							<Controller
								control={control}
								render={({ field }) => (
									<NumberInput
										rightSection={<Fragment />}
										rightSectionWidth={0}
										style={{ width: rem(56) }}
										onBlur={field.onBlur}
										value={field.value}
										onChange={(it) => field.onChange({ target: { value: it } })}
										error={Boolean(formState.errors.width)}
									/>
								)}
								name='height'
							/>
						</Flex>
						<Checkbox label='Fullscreen' {...register('fullscreen')} />
					</Flex>
					<Input.Wrapper label={`Ram Allocation [${watch('alloc')}M]`}>
						<Controller
							control={control}
							render={({ field }) => (
								<Slider
									min={512}
									max={16384}
									onChange={(it) => field.onChange({ target: { value: it } })}
									value={field.value}
									onBlur={field.onBlur}
								/>
							)}
							name='alloc'
						/>
					</Input.Wrapper>
					<Input.Wrapper
						label='Extra arguments'
						error={formState.errors.extraArgs?.message}
						description='You may want to add extra Java arguments'
					>
						<Input {...register('extraArgs')} error={Boolean(formState.errors.extraArgs)} />
					</Input.Wrapper>
				</Stack>
				<div />
				<Flex gap={8} justify='end'>
					<Button variant='default' type='button' onClick={close}>
						Cancel
					</Button>
					<Button type='submit'>Save</Button>
				</Flex>
			</Flex>
		</Drawer>
	)
}
