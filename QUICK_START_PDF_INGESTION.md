# 🚀 Quick Start: PDF Ingestion Pipeline

## Get Started in 5 Minutes

### Prerequisites

1. **Anthropic API Key** - Get one at https://console.anthropic.com/
2. **BuildOps 4.0 installed** - Follow `INSTALLATION.md`

---

## Step 1: Install Dependencies

```bash
cd /Users/billyribeiro/Documents/projects/build-ops
npm install
```

This installs the new `pdfjs-dist` dependency.

---

## Step 2: Run Cargo Build

```bash
cd src-tauri
cargo build
```

This compiles the new Rust dependencies:
- `pdf-extract` - PDF text extraction
- `lopdf` - PDF parsing
- `reqwest` - HTTP client for Anthropic API
- `tiktoken-rs` - Token counting
- `aes-gcm` - Encryption

---

## Step 3: Run the Application

```bash
npm run tauri dev
```

---

## Step 4: Import Your First Curriculum

### Option A: Single PDF

1. Navigate to **`/import`** in the app
2. Click **"Browse Files"** or drag a PDF
3. Click **"🔑 Enter API Key"**
4. Paste your Anthropic API key
5. Click **"Start Import"**
6. Wait for processing (2-5 minutes for typical PDF)
7. Review the generated plan
8. Click **"✓ Apply Import"**

### Option B: Multiple Files

1. Navigate to **`/import`**
2. Select multiple PDFs/Markdown files
3. Enter API key
4. Start import
5. Files are merged and analyzed together

---

## What Happens During Import?

```
📄 Extraction (10-30s)
   ↓ Text extracted from PDF/Markdown/Text
   
🤖 AI Analysis (1-3 min)
   ↓ Claude analyzes content structure
   
⚙️ Plan Generation (5-10s)
   ↓ Validates and structures curriculum
   
✅ Review Ready
   ↓ You can edit before applying
   
💾 Applied to Database
   ↓ Program created with all days, modules, etc.
```

---

## Example: Import a Svelte Tutorial PDF

Let's say you have `svelte-tutorial.pdf` (50 pages):

1. **Upload**: Drag `svelte-tutorial.pdf` to the upload zone
2. **API Key**: Enter `sk-ant-api03-...`
3. **Start**: Click "Start Import"
4. **Progress**: Watch extraction → analysis → generation
5. **Result**: 
   - Program: "Svelte 5 Mastery"
   - 3 Modules: Foundations, Reactivity, Advanced
   - 15 Days with specific builds
   - 45 Checklist items
   - 30 Quiz questions
   - 25 Concept tags
6. **Review**: Edit titles, descriptions if needed
7. **Apply**: Click "✓ Apply Import"
8. **Done**: Program appears in your programs list!

---

## Cost Estimate

**Anthropic API Pricing** (as of Feb 2026):
- Claude Sonnet 4: $3 per million input tokens, $15 per million output tokens

**Typical Import Costs:**
- Small PDF (10 pages): ~$0.05
- Medium PDF (50 pages): ~$0.20
- Large PDF (200 pages): ~$0.80
- Multi-file (5 files): ~$0.30

---

## Troubleshooting

### "Cannot find module '@tauri-apps/api/core'"

**Solution**: Install Tauri dependencies
```bash
npm install @tauri-apps/api@^2.2.0
```

### "Failed to extract PDF"

**Causes:**
- Image-only PDF (no text layer)
- Corrupted file
- Unsupported encryption

**Solutions:**
- Use OCR to convert image PDF to text
- Try a different PDF
- Use Markdown instead

### "API returned error 401"

**Cause**: Invalid API key

**Solution**: 
1. Go to https://console.anthropic.com/
2. Generate new API key
3. Copy the full key (starts with `sk-ant-`)
4. Paste in the import form

### "Import stuck in 'analyzing'"

**Causes:**
- Network timeout
- API rate limit
- Very large document

**Solutions:**
1. Cancel import
2. Wait 1 minute
3. Retry with same file
4. If still fails, split PDF into smaller files

---

## Tips for Best Results

### 1. **Prepare Your Documents**

✅ **Good:**
- Well-structured PDFs with clear headings
- Markdown files with proper formatting
- Text files with section breaks

❌ **Avoid:**
- Image-only PDFs
- Scanned documents without OCR
- Files with excessive formatting

### 2. **Optimize for AI Analysis**

✅ **Include:**
- Clear learning objectives
- Code examples
- Step-by-step instructions
- Practice exercises

❌ **Avoid:**
- Pure reference documentation
- Books without structure
- Marketing materials

### 3. **Review Before Applying**

Always review the generated plan:
- Check day titles are specific
- Verify time estimates are reasonable
- Ensure dependencies make sense
- Edit any AI hallucinations

### 4. **Multi-file Strategy**

For large curriculums:
- Split into logical sections
- Import separately or together
- Use descriptive filenames
- Keep related content together

---

## Advanced Usage

### Custom Prompts (Future Feature)

Currently, the AI prompt is fixed. Future versions will support:
- Custom curriculum templates
- Adjustable complexity levels
- Domain-specific instructions

### Batch Import (Future Feature)

Import multiple programs at once:
- Queue multiple imports
- Process in parallel
- Bulk apply

### Export Generated Plans

Save generated plans before applying:
```typescript
const plan = await imports.getImportPreview(jobId);
const json = JSON.stringify(plan, null, 2);
// Save to file
```

---

## Next Steps

1. **Import your first curriculum** - Try with a small PDF
2. **Review the generated plan** - See what AI creates
3. **Apply and test** - Start learning with the program
4. **Iterate** - Import more content, refine your approach

---

## Support

- **Documentation**: `PDF_INGESTION_IMPLEMENTATION.md`
- **Full Spec**: `BuildOps40_PDF_Ingestion_Pipeline.md`
- **General Setup**: `INSTALLATION.md`

---

## What's Included

### Backend (Rust)
- ✅ PDF/Markdown/Text extraction
- ✅ Smart document chunking
- ✅ Anthropic Claude API integration
- ✅ Plan validation and generation
- ✅ Transactional database import
- ✅ Error recovery and retry

### Frontend (SvelteKit)
- ✅ Drag-and-drop file upload
- ✅ Real-time progress tracking
- ✅ Full-featured review interface
- ✅ Secure API key handling
- ✅ Recent imports list

### Database
- ✅ `import_jobs` table with full history
- ✅ Tracks all pipeline stages
- ✅ Stores extracted content and AI responses

---

**Ready to transform your PDFs into structured learning programs? Let's go! 🚀**
