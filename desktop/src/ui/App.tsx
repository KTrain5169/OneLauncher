import type { ParentProps } from 'solid-js';
import { OverlayScrollbarsComponent } from 'overlayscrollbars-solid';
import environment from '../utils/environment';
import WindowFrame from './components/WindowFrame';
import Navbar from './components/Navbar';
import ErrorBoundary from './components/ErrorBoundary';
import AnimatedRoutes from './components/AnimatedRoutes';
import NotificationOverlay from './components/overlay/notifications/NotificationOverlay';

function App(props: ParentProps) {
	if (!environment.isDev()) {
		document.addEventListener('contextmenu', (event) => {
			event.preventDefault();
		});
	}

	return (
		<main class="flex flex-col bg-primary w-full min-h-screen overflow-hidden h-screen max-h-screen text-fg-primary">
			<ErrorBoundary><WindowFrame /></ErrorBoundary>
			<div class="flex flex-col px-8">
				<Navbar />
			</div>

			<div class="relative h-full w-full overflow-hidden">
				<div class="absolute top-0 left-0 flex flex-col h-full w-full overflow-x-hidden">
					<ErrorBoundary>
						<OverlayScrollbarsComponent class="os-hide-horizontal-scrollbar absolute top-0 left-0 flex flex-col h-full w-full overflow-x-hidden overflow-y-auto px-8 pb-8">
							<AnimatedRoutes>
								{props.children}
							</AnimatedRoutes>
						</OverlayScrollbarsComponent>
					</ErrorBoundary>
				</div>
			</div>

			<NotificationOverlay />
		</main>
	);
}

export default App;
