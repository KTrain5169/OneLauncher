import { type Params, useSearchParams } from '@solidjs/router';
import { Download01Icon } from '@untitled-theme/icons-solid';
import { onMount } from 'solid-js';
import { bridge } from '~imports';
import Button from '~ui/components/base/Button';
import ModCard from '~ui/components/content/ModCard';
import useCommand from '~ui/hooks/useCommand';

interface BrowserModParams extends Params {
	id: string;
	provider: string;
}

// TODO: Refactor the entire browser page and subpages
function BrowserMod() {
	const [params] = useSearchParams<BrowserModParams>();
	// const _isInvalid = !params.id || !params.provider;
	// const [cluster, setCluster] = createSignal<string>();
	// const [clusters, setClusters] = createSignal<[string, string][]>();
	const [pkg] = useCommand(bridge.commands.getMod, params.id!);

	function installTo() {
		// setVisible(true);
	}

	// function download() {
	// setVisible(false);
	// tryResult(bridge.commands.downloadMod, cluster()!, params.id!).then().catch(err => console.error(err));
	// }

	// function onChange(selected: number) {
	// setCluster(clusters()![selected]![1]);
	// }

	onMount(() => {
		// tryResult(bridge.commands.getClusters).then((res) => {
		// 	const list: [string, string][] = res.map(cluster => [cluster.meta.name, cluster.uuid]);
		// 	setClusters(list);
		// 	setCluster(clusters()![0]![1]);
		// });
	});

	return (
		<>
			<div class="flex flex-row gap-x-12">
				<ModCard {...pkg()!} />
				<div class="flex flex-1 flex-col items-start justify-between rounded-lg bg-component-bg p-4 px-6">
					<div class="w-full flex flex-row items-center justify-between">
						<h1>{pkg()?.title}</h1>
						<Button
							buttonStyle="primary"
							iconLeft={<Download01Icon />}
							children="Install to..."
							onClick={installTo}
						/>
					</div>
					<p>TO DO</p>
				</div>
			</div>
		</>
	);
}

// function InstallToCluster(props: CreateModal) {
// 	return (
// 		<Modal.Simple
// 			{...props}
// 			title="Install To..."
// 			buttons={[
// 				<Button
// 					buttonStyle="secondary"
// 					children="Cancel"
// 					onClick={() => props.hide()}
// 				/>,
// 				<Button
// 					buttonStyle="primary"
// 					children="Download"
// 					onClick={download}
// 				/>,
// 			]}
// 		>
// 			<Dropdown onChange={onChange}>
// 				<For each={clusters()}>
// 					{cluster => (
// 						<Dropdown.Row>{cluster[0]}</Dropdown.Row>
// 					)}
// 				</For>
// 			</Dropdown>
// 		</Modal.Simple>
// 	);
// }

BrowserMod.getUrl = function (params: BrowserModParams): string {
	return `/browser/mod?id=${params.id}&provider=${params.provider}`;
};

export default BrowserMod;
