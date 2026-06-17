+++
title = "Hashicorp Terraform"
date = "2022-11-28"
tags = ["terraform", "iac", "devops"]
excerpt = "Learn Infrastructure as Code with Terraform 鈥?state management, modules, workspaces, and best practices for provisioning cloud resources."
+++

Terraform by HashiCorp is the industry standard for Infrastructure as Code. It lets you define and provision infrastructure using a declarative configuration language called HCL.

## Core Concepts

- **Providers** 鈥?plugins that interact with cloud APIs
- **Resources** 鈥?infrastructure components (VMs, networks, etc.)
- **State** 鈥?mapping between config and real-world resources
- **Modules** 鈥?reusable configuration packages

## Basic Configuration

```hcl

terraform {
    required_providers {
        aws = {
            source  = "hashicorp/aws"
            version = "~> 4.0"
        }
    }
}

provider "aws" {
    region = "us-west-2"
}

resource "aws_instance" "web" {
    ami           = "ami-0c55b159cbfafe1f0"
    instance_type = "t2.micro"

    tags = {
        Name = "WebServer"
    }
}
```

## State Management

State files contain sensitive information. Never commit them to Git. Use remote backends instead:


```hcl
terraform {
  backend "s3" {
    bucket         = "my-terraform-state"
    key            = "prod/terraform.tfstate"
    region         = "us-west-2"
    dynamodb_table = "terraform-locks"
  }
}
```

## Modules

Modules promote reuse across environments:

```hcl
module "vpc" {
    source = "terraform-aws-modules/vpc/aws"
    version = "3.14.0"

    name = "my-vpc"
    cidr = "10.0.0.0/16"

    azs             = ["us-west-2a", "us-west-2b"]
    private_subnets = ["10.0.1.0/24", "10.0.2.0/24"]
    public_subnets  = ["10.0.101.0/24", "10.0.102.0/24"]

    enable_nat_gateway = true
    enable_vpn_gateway = true

    tags = {
        Environment = "production"
    }
}

```
## Workspaces

Workspaces manage multiple environments with the same configuration:

```bash
terraform workspace new staging
terraform workspace new production
terraform workspace select production
terraform plan
```

## Common Workflow

```bash
# Initialize providers and modules
terraform init

# Format and validate
terraform fmt
terraform validate

# See what will change
terraform plan

# Apply changes
terraform apply

# Destroy resources
terraform destroy
```

## Best Practices

- Use `terraform fmt` before every commit
- Pin provider and module versions
- Separate configuration from state
- Use workspaces or directories for environments
- Run `terraform plan` in CI pipelines

| Practice | Reason |
|----------|--------|
| Remote state | Collaboration and locking |
| Version pinning | Reproducible builds |
| Modules | DRY configuration |
| Plan in CI | Catch errors early |

Terraform brings the software engineering practices of version control and code review to infrastructure management.
