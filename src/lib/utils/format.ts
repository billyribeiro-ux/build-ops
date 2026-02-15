import { format, formatDistanceToNow, parseISO, isValid } from 'date-fns';

export function formatDate(dateStr: string): string {
	const date = parseISO(dateStr);
	if (!isValid(date)) return dateStr;
	return format(date, 'MMM d, yyyy');
}

export function formatDateTime(dateStr: string): string {
	const date = parseISO(dateStr);
	if (!isValid(date)) return dateStr;
	return format(date, 'MMM d, yyyy h:mm a');
}

export function formatTime(dateStr: string): string {
	const date = parseISO(dateStr);
	if (!isValid(date)) return dateStr;
	return format(date, 'h:mm a');
}

export function formatRelative(dateStr: string): string {
	const date = parseISO(dateStr);
	if (!isValid(date)) return dateStr;
	return formatDistanceToNow(date, { addSuffix: true });
}

export function formatMinutes(minutes: number): string {
	if (minutes < 60) return `${minutes}m`;
	const h = Math.floor(minutes / 60);
	const m = minutes % 60;
	return m > 0 ? `${h}h ${m}m` : `${h}h`;
}

export function formatScore(score: number, max: number = 100): string {
	return `${score}/${max}`;
}

export function formatPercent(value: number, decimals: number = 0): string {
	return `${value.toFixed(decimals)}%`;
}

export function formatNumber(n: number): string {
	return n.toLocaleString();
}

export function formatTimerDisplay(totalSeconds: number): string {
	const hours = Math.floor(totalSeconds / 3600);
	const minutes = Math.floor((totalSeconds % 3600) / 60);
	const seconds = totalSeconds % 60;
	const pad = (n: number) => n.toString().padStart(2, '0');
	if (hours > 0) return `${hours}:${pad(minutes)}:${pad(seconds)}`;
	return `${pad(minutes)}:${pad(seconds)}`;
}
