export interface KeyBinding {
	key: string;
	meta?: boolean;
	ctrl?: boolean;
	shift?: boolean;
	alt?: boolean;
	handler: () => void;
	description: string;
}

export function matchesBinding(event: KeyboardEvent, binding: KeyBinding): boolean {
	if (event.key.toLowerCase() !== binding.key.toLowerCase()) return false;
	if (binding.meta && !event.metaKey) return false;
	if (binding.ctrl && !event.ctrlKey) return false;
	if (binding.shift && !event.shiftKey) return false;
	if (binding.alt && !event.altKey) return false;
	return true;
}

export function handleKeyBindings(event: KeyboardEvent, bindings: KeyBinding[]): boolean {
	for (const binding of bindings) {
		if (matchesBinding(event, binding)) {
			event.preventDefault();
			event.stopPropagation();
			binding.handler();
			return true;
		}
	}
	return false;
}

export const isMac = typeof navigator !== 'undefined' && navigator.platform.includes('Mac');

export function modKey(key: string): string {
	return isMac ? `⌘${key}` : `Ctrl+${key}`;
}
