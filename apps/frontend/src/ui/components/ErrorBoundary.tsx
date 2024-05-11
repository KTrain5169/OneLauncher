import type { ParentProps } from 'solid-js';

// TODO: Uncomment code
function ErrorBoundary(props: ParentProps) {
	return (
		<>{props.children}</>
	// <Capturer
	// 	fallback={err => (
	// 		<div class="p-2 bg-danger/20 rounded-md border-danger border">
	// 			<h2>An unexpected error has occurred</h2>
	// 			<p>{err.toString()}</p>
	// 		</div>
	// 	)}
	// >
	// 	{props.children}
	// </Capturer>
	);
}

export default ErrorBoundary;
