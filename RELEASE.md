```bash
git add --all
git commit -sm "Release v1.0.0"
git push origin main

git tag -l | xargs git tag -d
git fetch --tags
git tag v1.0.0
git push origin v1.0.0
```
