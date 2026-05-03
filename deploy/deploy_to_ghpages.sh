#!/bin/bash
echo "🜏 Deploying to GitHub Pages..."
echo ""
read -p "Enter your GitHub username: " USERNAME
read -p "Enter repository name (default: verifykit-wasm): " REPO
REPO=${REPO:-verifykit-wasm}

echo ""
echo "Creating repository: $USERNAME/$REPO"
echo ""

# Initialize git
git init
git add .
git commit -m "Deploy VerifyKit WASM verifier v1.0"

# Create gh-pages branch
git branch -M gh-pages

# Add remote
git remote add origin https://github.com/$USERNAME/$REPO.git

# Push
git push -u origin gh-pages --force

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ DEPLOYMENT COMPLETE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "🌐 Live URL: https://$USERNAME.github.io/$REPO/"
echo ""
echo "📌 Next steps:"
echo "   1. Go to GitHub repo → Settings → Pages"
echo "   2. Verify source is set to 'gh-pages' branch"
echo "   3. Wait 1-2 minutes for deployment"
echo ""
