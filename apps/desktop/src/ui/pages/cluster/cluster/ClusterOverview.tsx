import { PlayIcon, Share07Icon } from '@untitled-theme/icons-solid';
import ClusterCover from '../../../components/game/ClusterCover';
import LoaderIcon from '../../../components/game/LoaderIcon';
import Button from '../../../components/base/Button';
import ScrollableContainer from '~ui/components/ScrollableContainer';
import Sidebar from '~ui/components/Sidebar';
import type { Cluster } from '~bindings';
import { upperFirst } from '~utils/primitives';
import useClusterContext from '~ui/hooks/useCluster';

function ClusterOverview() {
	const cluster = useClusterContext();

	return (
		<Sidebar.Page>
			<h1>Overview</h1>
			<ScrollableContainer>
				<Banner {...cluster} />
			</ScrollableContainer>
		</Sidebar.Page>
	);
}

function Banner(props: Cluster) {
	return (
		<div class="flex flex-row bg-component-bg rounded-xl p-2.5 gap-x-2.5">
			<div class="w-48 rounded-lg overflow-hidden border border-gray-10">
				<ClusterCover cluster={props} />
			</div>

			<div class="flex flex-col flex-1 text-fg-primary">
				<div class="flex-1">
					<h2 class="text-2xl">{props.meta.name}</h2>
					<span class="flex flex-row items-center gap-x-1">
						<LoaderIcon loader={props.meta.loader} class="w-5" />
						<span>{upperFirst(props.meta.loader || 'unknown')}</span>
						{props.meta.loader_version && <span>{props.meta.loader_version.id}</span>}
						<span>{props.meta.mc_version}</span>
					</span>
				</div>
				<span class="text-xs text-fg-secondary">
					Played for
					{' '}
					<b>{((props.meta.overall_played || 0n)).toString()}</b>
					{' '}
					hours
					{/* TODO: Ask Pauline if this is measured in seconds or milliseconds */}
				</span>
			</div>

			<div class="flex flex-row items-end gap-x-2.5 *:h-8">
				<Button
					buttonStyle="iconSecondary"
					children={<Share07Icon />}
				/>
				<Button
					buttonStyle="primary"
					iconLeft={<PlayIcon />}
					children="Launch"
					class="!w-auto"
					onClick={() => {}}
				/>
			</div>
		</div>
	);
}

export default ClusterOverview;