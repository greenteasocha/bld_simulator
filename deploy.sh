#!/bin/bash

# Cross Solver - Deploy Script
# WASMビルドからCDKデプロイまでを一括実行

set -e

echo "🎲 Cross Solver - Deployment Script"
echo "===================================="
echo ""

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

# Step 1: Build WASM
echo "📦 Step 1: Building WASM..."
wasm-pack build --target web
echo "✅ WASM build complete"
echo ""

# Step 2: Build CDK
echo "🔨 Step 2: Building CDK TypeScript..."
cd infra
npm run build
echo "✅ CDK build complete"
echo ""

# Step 3: Deploy with CDK
echo "🚀 Step 3: Deploying to AWS..."
npm run deploy -- "$@"
echo ""

echo "🎉 Deployment complete!"
echo ""
echo "📝 Next steps:"
echo "  1. Wait for DNS propagation (may take a few minutes)"
echo "  2. Visit your website URL (shown in the outputs above)"
echo "  3. Check CloudWatch Logs if any issues occur"
