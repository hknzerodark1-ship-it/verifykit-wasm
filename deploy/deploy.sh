#!/bin/bash
echo "🜏 Deploying to GitHub Pages..."
read -p "Enter your GitHub username: " USERNAME
REPO="verifykit-wasm"

git init
git add .
git commit -m "Deploy VerifyKit WASM verifier v1.0 - production grade"
git branch -M gh-pages
git remote add origin https://github.com/$USERNAME/$REPO.git
git push -u origin gh-pages --force

echo ""
echo "✅ Deployed to: https://$USERNAME.github.io/$REPO/"
echo ""
echo "📌 Go to GitHub Settings → Pages → Branch: gh-pages"
