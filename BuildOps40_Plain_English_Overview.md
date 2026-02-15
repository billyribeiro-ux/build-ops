# BuildOps 40 — What This App Does

Written in plain English for anyone who needs to understand the product.

---

## The Problem

You have a serious learning plan. Maybe it's 40 days of mastering a new programming framework. Maybe it's 90 days of leveling up across multiple technologies. You've got PDFs, roadmaps, exercises, projects — real curriculum that someone spent time building.

But right now, all of that lives in scattered documents. You open a PDF, read it, maybe build something, close your laptop, and tomorrow you're not sure where you left off, what you actually retained, or whether you're truly getting better. There's no system holding you accountable to the quality of your learning — just a vague sense of "I think I studied that."

BuildOps 40 fixes that.

---

## What It Is

BuildOps 40 is a desktop app for macOS that turns any learning curriculum into a structured daily execution system. Think of it as a mission control center for your education. It doesn't just track whether you showed up — it tracks whether you actually learned, whether you can prove it, and whether you can do it again from memory.

It runs entirely on your machine. No cloud accounts, no subscriptions, no sending your data anywhere. Your learning journal, your code, your scores, your files — all of it lives locally on your Mac.

---

## How It Works

### You Start With a Program

A program is your big picture plan. "SvelteKit Mastery" or "Rust Backend Engineering" or "Pine Script Indicator Development." You give it a name and a description, and it becomes the container for everything that follows.

Inside a program, you create modules. Modules are logical groupings — like chapters in a book. "HTML Foundations" might be your first module, "CSS Layout" your second, "JavaScript Fundamentals" your third, and so on. You can have as many modules as you want, and each module gets its own color so you can visually distinguish them.

Inside each module, you create day plans. A day plan is a single day's mission. It has a title, a list of syntax or concepts you need to learn, a description of what you're going to build, the files you should create, the criteria that define success, a stretch challenge if you want to push further, and any notes or tips.

Each day plan also has a checklist of specific tasks, quiz questions to test your understanding, and concept tags that label exactly which skills the day covers. You can also set dependencies — if Day 15 requires that you passed Day 9, the app enforces that.

There is no limit. You can have 40 days, 100 days, 200 days. You can add new days at any time. You can create entirely new programs for different subjects. The system grows with you.

### Or You Upload Your Curriculum

Here's where it gets powerful. If you already have your learning plan as PDF files, markdown documents, or text files, you don't have to manually type all of that into the app. You drag your files onto the import screen, the app reads them, sends the content to an AI that understands curriculum structure, and automatically generates your entire program — complete with modules, day plans, syntax targets, checklists, quiz questions, concept tags, and dependencies.

Before anything is saved, you get a full review screen where you can edit every single field. Change a title, add a quiz question, remove a checklist item, adjust the estimated time. Once you're happy with it, you hit apply, and your entire curriculum is ready to go. A 50-page PDF turns into a structured, executable learning program in under three minutes.

---

## The Daily Workflow

### Starting Your Day

When you open the app, you land on the dashboard. Front and center is today's mission card — it shows you which day you're on, what the mission is, and a button to start. Around it you see your overall progress as a ring chart, your current streak count, a trend line of your recent scores, and any alerts about days where you're stuck.

If the app's spaced repetition system has flagged concepts that are due for review, you'll see that too. These are things you learned days or weeks ago that the system wants you to revisit before you forget them.

### Doing the Work

When you start a day's attempt, the screen splits into two. On the left is your mission brief — everything you need to know about what to learn and build today. On the right is your workspace with several tabs.

The first tab is where you actually write code and take notes. There's a real code editor built into the app with syntax highlighting for every language you'd work with — JavaScript, TypeScript, HTML, CSS, Rust, Python, Svelte, and more. You write your exercises right here. No need to switch to another app unless you want to. You can create multiple code blocks and notes, reorder them, and everything autosaves every five seconds so you never lose work.

The second tab is your checklist. As you complete tasks from the day plan, you check them off. The progress bar fills up.

The third tab is your bug log. Every time you hit a problem, you log it — what happened, what caused it, how you fixed it, and what you'd do to prevent it next time. This sounds tedious but it becomes your most valuable reference material over time. Months from now, when you hit a similar problem, you can search your bug logs and find your own past solutions.

The fourth tab is evidence. You can upload screenshots, attach links to your GitHub commits or repos, drop in files, paste code snippets, or write markdown notes. This is your proof-of-work — evidence that you actually built the thing, not just read about it.

The fifth tab is the quiz. Each day has questions that test your understanding. Some are short answer, some are multiple choice, some ask you to write code, and some are reflection prompts. There's a timer per question to keep you honest.

At the bottom of the screen, a session timer runs the whole time you're working, and a small indicator shows when your work was last autosaved.

### The Memory Rebuild

This is the feature that separates real learning from the illusion of learning.

After you finish your guided implementation, the app challenges you to rebuild the same thing from memory — no notes, no reference material, just you and a blank editor with a 15-minute timer. If you can rebuild it, you actually learned it. If you can't, the day gets flagged regardless of how well your first build went.

This is uncomfortable and that's the point. It prevents you from fooling yourself into thinking you know something just because you followed along with instructions.

### Submitting Your Score

When you're done, you score yourself across five categories. Implementation completeness counts for 40 points — did you build everything the mission asked for? Code quality is 20 points — is your code clean, well-structured, and following best practices? Accessibility and UX is 15 points — does it work for everyone and feel polished? Performance and resilience is 15 points — is it fast, does it handle errors, does it degrade gracefully? And the quiz counts for 10 points.

That gives you a score out of 100. If you score below 70, the day is marked as blocked. You can't advance to any day that depends on this one until you replay it and score higher. Between 70 and 84 is a pass with some recommendations. Between 85 and 94 is a strong pass. And 95 or above is mastery level.

If your memory rebuild failed, the day is blocked no matter what your score was.

---

## What the App Tracks Over Time

### Streaks

The app tracks how many consecutive days you've completed work. There's a current streak counter and a longest-ever streak record. If life happens and you need to miss a day, you get two streak freezes per month — use one and your streak survives. But you only get two, so use them wisely.

### Badges

As you hit milestones, you earn badges. Complete your first day, that's a badge. Hit a 7-day streak, that's a badge. Score 95 or higher on a day, that's a mastery badge. Complete an entire program, that's a badge. Log 100 bugs, that's a badge. Pass 10 consecutive memory rebuilds, that's a badge. They're not just decorative — they mark real accomplishments.

### Skill Radar

The app builds a radar chart of your competency across every domain you're studying. If your program covers HTML, CSS, JavaScript, Svelte, SvelteKit, TypeScript, and animation, you'll see a radar with axes for each one. The scores are computed from your actual day attempt results and concept tag performance, not from self-assessment. Over time, you can see which domains are strong and which need more work.

### Concept Heatmap

Every day plan is tagged with specific concepts. The heatmap shows you a grid of all your concepts with color intensity based on how well you've scored on them. Green means strong, yellow means okay, red means you're struggling. At a glance, you know exactly where your gaps are.

### Burndown Chart

If your program has 40 days and you're on day 18, the burndown chart shows you how many days remain versus your current pace. If you're ahead of schedule, the line is below the ideal. If you're falling behind, it's above. This helps you plan your time realistically.

### Score Trends

A line chart showing your total score for every day attempt over time. You can see if you're improving, plateauing, or dipping. The trend line smooths out individual variance so you can see the real trajectory.

### Time Tracking

The app logs how long you spend on each day — both planned time and actual time. Over weeks, you can see if you're consistently underestimating or overestimating how long things take, and adjust your planning accordingly.

### Bug Pattern Analysis

Your bug logs aren't just for reference — the app analyzes them. It shows you which categories of bugs you hit most often, how severe they tend to be, and whether certain types keep recurring. If you keep making the same kind of mistake, the app makes that visible so you can address the root cause.

---

## Weekly Reviews

Every seven days, the app auto-generates a weekly review. It summarizes how many days you completed, your average score, your best and worst performances, total time invested, which concepts were strong, which were weak, and which days it recommends you replay.

There's also space for you to write your own notes — what went well this week, what you want to improve, and your goals for next week. This becomes a running journal of your growth.

---

## Spaced Repetition

The app uses a scientifically validated algorithm to schedule concept reviews at optimal intervals. When you complete a day and score well, the system schedules a review for a few days later. If you do well on the review, it pushes the next one further out — maybe a week, then two weeks, then a month. If you struggle, it brings the review closer.

This means the app actively fights the forgetting curve. Concepts you learned in week one don't just fade away — the system keeps surfacing them at exactly the right time to reinforce long-term retention.

On your dashboard, you'll see a count of reviews due today, and you can knock them out before starting your main mission.

---

## Searching Everything

Every note you've written, every bug you've logged, every quiz answer, every code snippet, every reflection — it's all searchable. Hit Command-K and type a keyword. The app searches across your entire learning history and shows you results grouped by type with highlighted matches.

Six months into your journey, when you vaguely remember solving a specific CSS grid problem but can't remember which day it was, you search "grid template areas" and find your own notes, code, and bug logs instantly. This accumulated knowledge base becomes more valuable the longer you use the app.

---

## Exporting Your Work

Everything can be exported. You can generate a PDF report of your entire program — scores, reflections, analytics, everything — as a professional document. You can export weekly reports as PDFs. You can export your raw data as JSON, which can be re-imported into the app on another machine. And you can export your scores and attempts as a CSV spreadsheet.

You can also export individual day plan templates as JSON files, which means if you build a great curriculum structure, you can share it or reuse it as the starting point for a new program.

---

## The Settings

You can switch between dark and light themes. You can adjust the code editor's font size and color scheme. You can change the autosave interval, the default session time, the memory rebuild time limit, and the score thresholds for blocking and mastery. You can enable or disable daily reminders and spaced repetition. And you can manage your data — export everything, import data, or clear everything and start fresh.

---

## Who This Is For

This app is for anyone who takes structured learning seriously and wants more than a to-do list to manage it. It's built for the kind of person who knows that reading a tutorial isn't the same as building something, and that building something once isn't the same as understanding it deeply enough to rebuild it from memory.

It turns the messy, undisciplined process of self-education into a structured engineering operation with clear inputs, measurable outputs, and honest accountability.

---

## One Sentence

BuildOps 40 is a local-first macOS app that transforms any learning curriculum into a daily execution system that tracks not just whether you studied, but whether you actually learned — with scoring, evidence, memory testing, spaced repetition, and analytics that show you exactly where you stand.
