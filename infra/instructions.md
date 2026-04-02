## Setup steps on your other machine

1. **Create the storage account** (one-time):
   ```bash
   cd infra
   ./bootstrap-tfstate.sh 0efec2f7-a4de-4173-b979-cf0c52d78424
   ```

2. **Migrate your existing local state to remote**:
   ```bash
   terraform init -migrate-state
   ```
   It'll ask to copy state — say yes.

3. **Apply** to create the dev environment:
   ```bash
   terraform apply
   ```

4. **Add GitHub secrets** for the Terraform CI workflow. You'll need a service principal:
   ```bash
   az ad sp create-for-rbac --name "github-terraform" --role Contributor \
     --scopes /subscriptions/0efec2f7-a4de-4173-b979-cf0c52d78424
   ```
   Then add these as repo secrets: `ARM_CLIENT_ID`, `ARM_CLIENT_SECRET`, `ARM_SUBSCRIPTION_ID`, `ARM_TENANT_ID`.

5. Also add `AZURE_STATIC_WEB_APPS_DEV_API_TOKEN` from `terraform output dev_static_web_app_api_key`.
