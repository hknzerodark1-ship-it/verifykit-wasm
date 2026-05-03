#!/bin/bash
# Deploy to GitHub Pages
cd ~/citadel/verifykit-v1/wasm/deploy
git init
git add .
git commit -m "Deploy VerifyKit WASM verifier"
git branch -M gh-pages
git remote add origin https://github.com/YOUR_USERNAME/verifykit-wasm.git
git push -u origin gh-pages --force
echo "Deployed to: https://YOUR_USERNAME.github.io/verifykit-wasm/"
