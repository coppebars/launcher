import { ChangeEvent }  from 'react'

import { Flex }         from '@mantine/core'
import { Input }        from '@mantine/core'
import { useUnit }      from 'effector-react'

import { $nickname }    from '@entity/profile'
import { setNickname }  from '@entity/profile'
import { PaddedLayout } from '@layout/padded'

export function ProfilesPage() {
	const nickaname = useUnit($nickname)

	return (
		<PaddedLayout>
			<Flex w='100%' h='100%' justify='center' align='center'>
				<Input.Wrapper label='Local nickname'>
					<Input
						value={nickaname}
						onChange={({ currentTarget }: ChangeEvent<HTMLInputElement>) => setNickname(currentTarget.value)}
					/>
				</Input.Wrapper>
			</Flex>
		</PaddedLayout>
	)
}
