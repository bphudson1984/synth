variable "project_name" {
  description = "Name for the Static Web App and resource group prefix"
  type        = string
  default     = "prophet5-synth"
}

variable "location" {
  description = "Azure region"
  type        = string
  default     = "westeurope"
}

variable "subscription_id" {
  description = "Azure subscription ID"
  type        = string
}

variable "sku_tier" {
  description = "Static Web App SKU tier (Free or Standard)"
  type        = string
  default     = "Free"
}

variable "enable_qa" {
  description = "Whether to create a QA Static Web App environment"
  type        = bool
  default     = false
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default = {
    project     = "prophet5-synth"
    environment = "production"
    managed_by  = "terraform"
  }
}
