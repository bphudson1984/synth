terraform {
  required_version = ">= 1.5"

  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 4.0"
    }
  }

  backend "azurerm" {
    resource_group_name  = "tfstate-rg"
    storage_account_name = "tfstateprophet5"
    container_name       = "tfstate"
    key                  = "prophet5.tfstate"
  }
}

provider "azurerm" {
  features {}
  subscription_id = var.subscription_id
}

# ============================================================
# Resource Group
# ============================================================
resource "azurerm_resource_group" "prophet5" {
  name     = "${var.project_name}-rg"
  location = var.location
  tags     = var.tags
}

# ============================================================
# Azure Static Web App
# ============================================================
resource "azurerm_static_web_app" "prophet5" {
  name                = var.project_name
  resource_group_name = azurerm_resource_group.prophet5.name
  location            = var.location
  sku_tier            = var.sku_tier
  sku_size            = var.sku_tier == "Free" ? "Free" : "Standard"
  tags                = var.tags
}

# ============================================================
# Azure Static Web App — Dev Environment
# ============================================================
resource "azurerm_static_web_app" "orbit_dev" {
  name                = "orbit-dev"
  resource_group_name = azurerm_resource_group.prophet5.name
  location            = var.location
  sku_tier            = "Free"
  sku_size            = "Free"
  tags = merge(var.tags, {
    environment = "dev"
  })
}

# ============================================================
# Outputs — needed by GitHub Actions
# ============================================================
output "static_web_app_name" {
  value = azurerm_static_web_app.prophet5.name
}

output "static_web_app_url" {
  value = "https://${azurerm_static_web_app.prophet5.default_host_name}"
}

output "static_web_app_api_key" {
  value     = azurerm_static_web_app.prophet5.api_key
  sensitive = true
}

output "resource_group_name" {
  value = azurerm_resource_group.prophet5.name
}

# ============================================================
# Dev environment outputs
# ============================================================
output "dev_static_web_app_name" {
  value = azurerm_static_web_app.orbit_dev.name
}

output "dev_static_web_app_url" {
  value = "https://${azurerm_static_web_app.orbit_dev.default_host_name}"
}

output "dev_static_web_app_api_key" {
  value     = azurerm_static_web_app.orbit_dev.api_key
  sensitive = true
}
