#!/usr/bin/env bash
# One-time script to create the Azure Storage Account for Terraform remote state.
# Run this before switching to the azurerm backend.
#
# Usage: ./bootstrap-tfstate.sh <subscription_id>

set -euo pipefail

SUBSCRIPTION_ID="${1:?Usage: ./bootstrap-tfstate.sh <subscription_id>}"
RESOURCE_GROUP="tfstate-rg"
STORAGE_ACCOUNT="tfstateprophet5"
CONTAINER="tfstate"
LOCATION="uksouth"

az account set --subscription "$SUBSCRIPTION_ID"

echo "Creating resource group: $RESOURCE_GROUP"
az group create --name "$RESOURCE_GROUP" --location "$LOCATION"

echo "Creating storage account: $STORAGE_ACCOUNT"
az storage account create \
  --name "$STORAGE_ACCOUNT" \
  --resource-group "$RESOURCE_GROUP" \
  --location "$LOCATION" \
  --sku Standard_LRS \
  --min-tls-version TLS1_2 \
  --allow-blob-public-access false

echo "Creating blob container: $CONTAINER"
az storage container create \
  --name "$CONTAINER" \
  --account-name "$STORAGE_ACCOUNT"

echo ""
echo "Done! Now on the machine with existing state, run:"
echo "  cd infra"
echo "  terraform init -migrate-state"
echo ""
echo "This will upload your local state to the remote backend."
