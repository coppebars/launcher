/* eslint-disable react/jsx-no-useless-fragment, react/jsx-fragments */

import { Fragment }    from 'react'
import { useCallback } from 'react'

import { zodResolver } from '@hookform/resolvers/zod'
import { Button }      from '@mantine/core'
import { Slider }      from '@mantine/core'
import { rem }         from '@mantine/core'
import { NumberInput } from '@mantine/core'
import { Checkbox }    from '@mantine/core'
import { Divider }     from '@mantine/core'
import { Stack }       from '@mantine/core'
import { Drawer }      from '@mantine/core'
import { Flex }        from '@mantine/core'
import { Input }       from '@mantine/core'
import { IconX }       from '@tabler/icons-react'
import { Controller }  from 'react-hook-form'
import { useForm }     from 'react-hook-form'
import { z }           from 'zod'

export const schema = z.object({
	name: z.string().min(3).max(40),
	path: z.string(),
	width: z.number().min(800).max(3840),
	height: z.number().min(600).max(2160),
	fullscreen: z.boolean(),
	extraArgs: z.string(),
	alloc: z.number().min(512).max(16384),
})

interface Props {
	opened?: boolean
	onClose?: () => void
}

export function Form(props: Props) {
	// eslint-disable-next-line @typescript-eslint/no-empty-function
	const { opened = false, onClose = () => {} } = props

	const { register, formState, handleSubmit, control, watch } = useForm<z.infer<typeof schema>>({
		defaultValues: {
			name: '',
			path: '',
			width: 1280,
			height: 720,
			fullscreen: false,
			extraArgs: '',
			alloc: 2048,
		},
		reValidateMode: 'onChange',
		resolver: zodResolver(schema),
	})

	const submit = useCallback((data: z.infer<typeof schema>) => {
		console.log(data)
	}, [])

	return (
		<Drawer opened={opened} onClose={onClose} position='right'>
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
					<Button variant='default' type='button'>
						Cancel
					</Button>
					<Button type='submit'>Save</Button>
				</Flex>
			</Flex>
		</Drawer>
	)
}
