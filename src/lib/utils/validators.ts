export function required(value: string, fieldName: string): string | null {
	if (!value || !value.trim()) return `${fieldName} is required`;
	return null;
}

export function minLength(value: string, min: number, fieldName: string): string | null {
	if (value.length < min) return `${fieldName} must be at least ${min} characters`;
	return null;
}

export function maxLength(value: string, max: number, fieldName: string): string | null {
	if (value.length > max) return `${fieldName} must be at most ${max} characters`;
	return null;
}

export function inRange(value: number, min: number, max: number, fieldName: string): string | null {
	if (value < min || value > max) return `${fieldName} must be between ${min} and ${max}`;
	return null;
}

export function isPositive(value: number, fieldName: string): string | null {
	if (value <= 0) return `${fieldName} must be positive`;
	return null;
}

export function isUrl(value: string, fieldName: string): string | null {
	try {
		new URL(value);
		return null;
	} catch {
		return `${fieldName} must be a valid URL`;
	}
}

export type ValidationError = string | null;

export function validate(
	...checks: ValidationError[]
): string[] {
	return checks.filter((e): e is string => e !== null);
}
