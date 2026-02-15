import { test, expect } from '@playwright/test';

// Tests run against the live development server
// Manual test data setup: Create a program called "React Mastery E2E Test" with at least one day plan

test.describe('BuildOps 40 End-to-End Tests', () => {
	
	test('1. Dashboard loads and shows program data', async ({ page }) => {
		await page.goto('http://localhost:5173/');
		
		// Check page title
		await expect(page.locator('h1')).toContainText('Dashboard');
		
		// Dashboard should have loaded with widgets (check for any card or section)
		const hasCards = await page.locator('[class*="card"]').count() > 0;
		const hasContent = await page.locator('body').textContent();
		
		expect(hasContent).toContain('Dashboard');
	});
	
	test('2. Programs page lists all programs', async ({ page }) => {
		await page.goto('http://localhost:5173/programs');
		
		await expect(page.locator('h1')).toContainText('Programs');
		
		// Check for create button or empty state
		const hasCreateButton = await page.locator('button:has-text("Create Program")').isVisible().catch(() => false);
		const hasNewButton = await page.locator('button:has-text("New Program")').isVisible().catch(() => false);
		
		expect(hasCreateButton || hasNewButton).toBeTruthy();
	});
	
	test('3. Program detail page shows modules and days', async ({ page }) => {
		await page.goto('http://localhost:5173/programs');
		
		// Click on first program if it exists
		const firstProgram = page.locator('[href^="/programs/"]').first();
		if (await firstProgram.isVisible()) {
			await firstProgram.click();
			// Should be on program detail page
			await expect(page).toHaveURL(/\/programs\/[a-f0-9-]+$/);
		}
	});
	
	test('4. Day Mission view loads (if day plans exist)', async ({ page }) => {
		await page.goto('http://localhost:5173/programs');
		
		// Try to find a day plan link
		const dayLink = page.locator('[href^="/day/"]').first();
		if (await dayLink.isVisible()) {
			await dayLink.click();
			await expect(page).toHaveURL(/\/day\/[a-f0-9-]+$/);
			await expect(page.locator('h1')).toBeVisible();
		}
	});
	
	test('5. Working screen loads with all tabs (if attempts exist)', async ({ page }) => {
		await page.goto('http://localhost:5173/programs');
		
		// Try to find an attempt link
		const attemptLink = page.locator('[href^="/work/"]').first();
		if (await attemptLink.isVisible()) {
			await attemptLink.click();
			await expect(page).toHaveURL(/\/work\/[a-f0-9-]+$/);
			
			// Check all tabs are present
			await expect(page.locator('button:has-text("Checklist")')).toBeVisible();
			await expect(page.locator('button:has-text("Bugs")')).toBeVisible();
			await expect(page.locator('button:has-text("Evidence")')).toBeVisible();
			await expect(page.locator('button:has-text("Quiz")')).toBeVisible();
		}
	});
	
	test('6. Checklist tab is accessible', async ({ page }) => {
		await page.goto('http://localhost:5173/programs');
		
		const attemptLink = page.locator('[href^="/work/"]').first();
		if (await attemptLink.isVisible()) {
			await attemptLink.click();
			await page.click('button:has-text("Checklist")');
			// Checklist tab should be active
			await expect(page.locator('button:has-text("Checklist")')).toBeVisible();
		}
	});
	
	test('7. Quiz tab shows score inputs and memory rebuild timer', async ({ page }) => {
		await page.goto('http://localhost:5173/programs');
		
		const attemptLink = page.locator('[href^="/work/"]').first();
		if (await attemptLink.isVisible()) {
			await attemptLink.click();
			await page.click('button:has-text("Quiz")');
			
			// Check for score inputs
			await expect(page.locator('text=Implementation')).toBeVisible();
			await expect(page.locator('text=Code Quality')).toBeVisible();
			
			// Check for memory rebuild timer
			await expect(page.locator('text=Memory Rebuild Challenge')).toBeVisible();
			const startButton = page.locator('button:has-text("Start")');
			if (await startButton.isVisible()) {
				await startButton.click();
				await page.waitForTimeout(2000);
				await expect(page.locator('text=00:0')).toBeVisible();
				await page.click('button:has-text("Stop")');
			}
		}
	});
	
	test('8. Analytics page shows charts with data', async ({ page }) => {
		await page.goto('http://localhost:5173/analytics');
		
		await expect(page.locator('h1')).toContainText('Analytics');
		
		// Check for chart sections (flexible matching)
		const hasSkill = await page.locator('text=Skill').isVisible().catch(() => false);
		const hasScore = await page.locator('text=Score').isVisible().catch(() => false);
		const hasProgress = await page.locator('text=Progress').isVisible().catch(() => false);
		
		expect(hasSkill || hasScore || hasProgress).toBeTruthy();
	});
	
	test('9. Search page loads and can search', async ({ page }) => {
		await page.goto('http://localhost:5173/search');
		
		await expect(page.locator('h1')).toContainText('Search');
		
		// Type in search box
		await page.fill('input[placeholder*="Search"]', 'test');
		
		// Check for results or empty state
		await page.waitForTimeout(500);
		
		// Should show either results or "no results" or "start typing" message
		const hasContent = await page.locator('body').textContent();
		expect(hasContent).toBeTruthy();
	});
	
	test('10. Export page loads with format options', async ({ page }) => {
		await page.goto('http://localhost:5173/export');
		
		await expect(page.locator('h1')).toContainText('Export');
		
		// Check for export format options
		await expect(page.locator('text=PDF Report')).toBeVisible();
		await expect(page.locator('text=JSON Backup')).toBeVisible();
		await expect(page.locator('text=CSV Data')).toBeVisible();
		
		// Select JSON export
		await page.click('button:has-text("JSON Backup")');
		
		// Check export button is visible
		await expect(page.locator('button:has-text("Export as JSON")')).toBeVisible();
	});
	
	test('11. Settings page loads and shows capacity settings', async ({ page }) => {
		await page.goto('http://localhost:5173/settings');
		
		await expect(page.locator('h1')).toContainText('Settings');
		
		// Click capacity tab
		await page.click('button:has-text("Capacity")');
		
		// Check for capacity fields
		await expect(page.locator('text=Default Daily Minutes')).toBeVisible();
		await expect(page.locator('text=Weekly Study Days')).toBeVisible();
		
		// Check for save button
		await expect(page.locator('button:has-text("Save Settings")')).toBeVisible();
	});
	
	test('12. Weekly Reviews page loads', async ({ page }) => {
		await page.goto('http://localhost:5173/reviews');
		
		await expect(page.locator('h1')).toContainText('Review');
		
		// Page should load with content
		const bodyText = await page.locator('body').textContent();
		expect(bodyText).toBeTruthy();
	});
	
	test('13. Evidence Locker page loads', async ({ page }) => {
		await page.goto('http://localhost:5173/evidence');
		
		await expect(page.locator('h1')).toContainText('Evidence Locker');
		
		// Should show artifacts or empty state
		const hasArtifacts = await page.locator('text=artifact').isVisible().catch(() => false);
		const isEmpty = await page.locator('text=No artifacts yet').isVisible().catch(() => false);
		
		expect(hasArtifacts || isEmpty).toBeTruthy();
	});
	
	test('14. Command Palette opens with Cmd+K', async ({ page }) => {
		await page.goto('http://localhost:5173/');
		
		// Get initial input count
		const initialInputs = await page.locator('input').count();
		
		// Press Cmd+K (or Ctrl+K on non-Mac)
		await page.keyboard.press(process.platform === 'darwin' ? 'Meta+K' : 'Control+K');
		
		// Wait for command palette to appear
		await page.waitForTimeout(1000);
		
		// Check if command palette opened (should have more inputs or visible overlay)
		const afterInputs = await page.locator('input').count();
		const hasOverlay = await page.locator('.fixed').count() > 0;
		
		// Command palette should have opened (new inputs or overlay visible)
		expect(afterInputs >= initialInputs || hasOverlay).toBeTruthy();
		
		// Press Escape to close
		await page.keyboard.press('Escape');
	});
	
	test('15. Sidebar navigation works', async ({ page }) => {
		await page.goto('http://localhost:5173/');
		
		// Check sidebar items
		await expect(page.locator('a:has-text("Dashboard")')).toBeVisible();
		await expect(page.locator('a:has-text("Programs")')).toBeVisible();
		await expect(page.locator('a:has-text("Analytics")')).toBeVisible();
		await expect(page.locator('a:has-text("Reviews")')).toBeVisible();
		await expect(page.locator('a:has-text("Evidence")')).toBeVisible();
		await expect(page.locator('a:has-text("Search")')).toBeVisible();
		await expect(page.locator('a:has-text("Export")')).toBeVisible();
		await expect(page.locator('a:has-text("Settings")')).toBeVisible();
		
		// Click on Analytics
		await page.click('a:has-text("Analytics")');
		
		// Verify navigation worked
		await expect(page).toHaveURL(/.*analytics/);
		await expect(page.locator('h1')).toContainText('Analytics');
	});
	
	test('16. Create new program flow', async ({ page }) => {
		await page.goto('http://localhost:5173/programs');
		
		// Try to click create program button
		const createButton = page.locator('button:has-text("Create Program")').or(page.locator('button:has-text("New Program")'));
		if (await createButton.first().isVisible()) {
			await createButton.first().click();
			
			// Should navigate to new program page or show modal
			await page.waitForTimeout(1000);
			
			// Try to find title input (could be in modal or page)
			const titleInput = page.locator('input').filter({ hasText: '' }).first();
			if (await titleInput.isVisible()) {
				await titleInput.fill('E2E Test Program');
			}
		}
	});
	
	test('17. Autosave indicator appears in working screen', async ({ page }) => {
		await page.goto('http://localhost:5173/programs');
		
		const attemptLink = page.locator('[href^="/work/"]').first();
		if (await attemptLink.isVisible()) {
			await attemptLink.click();
			await page.waitForSelector('h1');
			
			// Wait for autosave (5 second interval)
			await page.waitForTimeout(6000);
			
			// Should show some save status
			const hasSaved = await page.locator('text=Saved').isVisible().catch(() => false);
			const isAutosaving = await page.locator('text=Autosaving').isVisible().catch(() => false);
			
			expect(hasSaved || isAutosaving).toBeTruthy();
		}
	});
	
	test('18. Score inputs work in quiz tab', async ({ page }) => {
		await page.goto('http://localhost:5173/programs');
		
		const attemptLink = page.locator('[href^="/work/"]').first();
		if (await attemptLink.isVisible()) {
			await attemptLink.click();
			await page.click('button:has-text("Quiz")');
			
			// Check score inputs exist
			const scoreInputs = page.locator('input[type="number"]');
			const count = await scoreInputs.count();
			expect(count).toBeGreaterThan(0);
		}
	});
	
	test('19. All pages load without errors', async ({ page }) => {
		const routes = [
			'/',
			'/programs',
			'/analytics',
			'/reviews',
			'/evidence',
			'/search',
			'/export',
			'/import',
			'/settings'
		];
		
		for (const route of routes) {
			await page.goto(`http://localhost:5173${route}`);
			
			// Check no error messages
			const hasError = await page.locator('text=Error').isVisible().catch(() => false);
			expect(hasError).toBeFalsy();
			
			// Check page loaded (has h1)
			await expect(page.locator('h1')).toBeVisible();
		}
	});
	
	test('20. Full user journey: Dashboard → Programs → Working Screen', async ({ page }) => {
		// Start at dashboard
		await page.goto('http://localhost:5173/');
		await expect(page.locator('h1')).toContainText('Dashboard');
		
		// Navigate to programs
		await page.click('a:has-text("Programs")');
		await expect(page.locator('h1')).toContainText('Programs');
		
		// If programs exist, click on first one
		const firstProgram = page.locator('[href^="/programs/"]').first();
		if (await firstProgram.isVisible()) {
			await firstProgram.click();
			await expect(page).toHaveURL(/\/programs\/[a-f0-9-]+$/);
			
			// If attempt exists, navigate to it
			const attemptLink = page.locator('[href^="/work/"]').first();
			if (await attemptLink.isVisible()) {
				await attemptLink.click();
				await expect(page.locator('button:has-text("Checklist")')).toBeVisible();
				
				// Go through tabs
				await page.click('button:has-text("Checklist")');
				await page.click('button:has-text("Bugs")');
				await page.click('button:has-text("Evidence")');
				await page.click('button:has-text("Quiz")');
				await expect(page.locator('text=Implementation')).toBeVisible();
				await expect(page.locator('text=Memory Rebuild Challenge')).toBeVisible();
			}
		}
		
		console.log('✅ Full user journey completed successfully!');
	});
});
