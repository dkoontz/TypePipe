#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Repository settings
REPO="dkoontz/TypeyPipe"
HOMEBREW_DIR="../homebrew-typeypipe"
FORMULA_FILE="$HOMEBREW_DIR/Formula/typeypipe.rb"

echo -e "${GREEN}🚀 Build, Release, and Update Homebrew for TypeyPipe${NC}"

# Check if gh CLI is available
if ! command -v gh &> /dev/null; then
    echo -e "${RED}❌ Error: GitHub CLI (gh) is not installed${NC}"
    echo "Please install it with: brew install gh"
    exit 1
fi

# Check if authenticated with GitHub
if ! gh auth status &> /dev/null; then
    echo -e "${RED}❌ Error: Not authenticated with GitHub${NC}"
    echo "Please run: gh auth login"
    exit 1
fi

# Check if homebrew directory exists
if [ ! -d "$HOMEBREW_DIR" ]; then
    echo -e "${RED}❌ Error: $HOMEBREW_DIR directory not found${NC}"
    echo "Please ensure the homebrew-typeypipe repository is cloned next to this directory"
    exit 1
fi

# Check if formula file exists
if [ ! -f "$FORMULA_FILE" ]; then
    echo -e "${RED}❌ Error: Formula file not found at $FORMULA_FILE${NC}"
    exit 1
fi

# Get version from Cargo.toml
echo -e "${YELLOW}📖 Reading version from Cargo.toml...${NC}"
VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

if [ -z "$VERSION" ]; then
    echo -e "${RED}❌ Error: Could not read version from Cargo.toml${NC}"
    exit 1
fi

echo -e "${GREEN}📦 Version from Cargo.toml: $VERSION${NC}"

# Check if release already exists
echo -e "${YELLOW}🔍 Checking if release v$VERSION already exists...${NC}"
if gh release view "v$VERSION" &> /dev/null; then
    echo -e "${BLUE}ℹ️ Release v$VERSION already exists, skipping build step${NC}"
else
    echo -e "${YELLOW}🏗️ Running GitHub workflow to build and release v$VERSION...${NC}"
    
    # Trigger the workflow
    gh workflow run release.yml --field version="$VERSION"
    
    echo -e "${YELLOW}⏳ Waiting for workflow to complete...${NC}"
    
    # Wait for the workflow to start (give it a few seconds)
    sleep 5
    
    # Get the most recent workflow run
    RUN_ID=$(gh run list --workflow=release.yml --limit=1 --json databaseId --jq '.[0].databaseId')
    
    if [ -z "$RUN_ID" ]; then
        echo -e "${RED}❌ Error: Could not find workflow run${NC}"
        exit 1
    fi
    
    echo -e "${BLUE}📊 Workflow run ID: $RUN_ID${NC}"
    
    # Wait for completion with status updates
    while true; do
        STATUS=$(gh run view "$RUN_ID" --json status --jq '.status')
        
        case $STATUS in
            "completed")
                CONCLUSION=$(gh run view "$RUN_ID" --json conclusion --jq '.conclusion')
                if [ "$CONCLUSION" = "success" ]; then
                    echo -e "${GREEN}✅ Workflow completed successfully!${NC}"
                    break
                else
                    echo -e "${RED}❌ Workflow failed with conclusion: $CONCLUSION${NC}"
                    echo -e "${RED}Check the workflow logs: gh run view $RUN_ID --log${NC}"
                    exit 1
                fi
                ;;
            "in_progress"|"queued")
                echo -e "${BLUE}⏳ Workflow status: $STATUS (waiting...)${NC}"
                sleep 30
                ;;
            *)
                echo -e "${RED}❌ Unexpected workflow status: $STATUS${NC}"
                exit 1
                ;;
        esac
    done
    
    echo -e "${GREEN}🎉 Release v$VERSION created successfully!${NC}"
fi

# Construct download URLs
MACOS_URL="https://github.com/$REPO/releases/download/v$VERSION/typeypipe-v$VERSION-macos-x64.tar.gz"
LINUX_URL="https://github.com/$REPO/releases/download/v$VERSION/typeypipe-v$VERSION-linux-x64.tar.gz"

echo -e "${YELLOW}🔍 Calculating SHA256 hashes...${NC}"

# Get SHA256 hashes
echo "  📥 Downloading macOS binary..."
MACOS_SHA256=$(curl -sL "$MACOS_URL" | shasum -a 256 | cut -d' ' -f1)

echo "  📥 Downloading Linux binary..."
LINUX_SHA256=$(curl -sL "$LINUX_URL" | shasum -a 256 | cut -d' ' -f1)

if [ -z "$MACOS_SHA256" ] || [ -z "$LINUX_SHA256" ]; then
    echo -e "${RED}❌ Error: Failed to calculate SHA256 hashes${NC}"
    exit 1
fi

echo -e "${GREEN}✅ macOS SHA256: $MACOS_SHA256${NC}"
echo -e "${GREEN}✅ Linux SHA256: $LINUX_SHA256${NC}"

# Update the formula file
echo -e "${YELLOW}📝 Updating formula file...${NC}"

# Create the new formula content
cat > "$FORMULA_FILE" << EOF
class Typeypipe < Formula
  desc "Shell wrapper that creates a mailbox system for external applications"
  homepage "https://github.com/$REPO"
  license "MIT"
  version "$VERSION"

  on_macos do
    url "$MACOS_URL"
    sha256 "$MACOS_SHA256"
  end

  on_linux do
    url "$LINUX_URL"
    sha256 "$LINUX_SHA256"
  end

  def install
    bin.install "typeypipe"
  end

  test do
    assert_match "typeypipe", shell_output("#{bin}/typeypipe --help")
  end
end
EOF

echo -e "${GREEN}✅ Formula file updated${NC}"

# Git operations
echo -e "${YELLOW}📤 Committing and pushing changes...${NC}"

cd "$HOMEBREW_DIR"

# Check if there are changes to commit
if git diff --quiet HEAD -- Formula/typeypipe.rb; then
    echo -e "${YELLOW}ℹ️ No changes detected in formula file${NC}"
    exit 0
fi

# Add, commit, and push
git add Formula/typeypipe.rb
git commit -m "Update typeypipe to v$VERSION

- Update macOS binary URL and SHA256
- Update Linux binary URL and SHA256
- Version: $VERSION"

git push origin main

echo -e "${GREEN}🎉 Successfully updated Homebrew formula to v$VERSION${NC}"
echo -e "${GREEN}Users can now install the latest version with:${NC}"
echo -e "${GREEN}  brew tap dkoontz/typeypipe${NC}"
echo -e "${GREEN}  brew install typeypipe${NC}"