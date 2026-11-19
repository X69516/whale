
# Whale Architecture

## Overview

Whale is designed around separation of responsibilities
and minimizing the amount of trusted code.

The architecture consists of two primary planes.

## Data Plane

The data plane is responsible for handling external traffic.

Primary application:
- serverd

Planned components:
- TCP transport
- TLS
- HTTP processing
- Routing
- Request dispatch

## Control Plane

The control plane is responsible for managing execution
and system resources.

Primary application:
- supervisord

Planned components:
- Worker lifecycle
- Process management
- Sandbox configuration
- Identity verification
- Audit

## Communication

The applications are intended to communicate through
a dedicated IPC mechanism.

IPC authentication and authorization are planned
as independent security requirements.

## Security Principles

- Zero Trust
- Least Privilege
- Default Deny
- Process Isolation
- Explicit Identity
- Resource Limits
- Auditable Decisions

## Current Implementation

Version 0.1.0 implements only the initial workspace,
application entrypoints and basic TCP lifecycle.

The architecture described above is the intended
design, not a claim that all components already exist.

