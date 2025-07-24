use anchor_lang::prelude::*;

declare_id!("BMWb6XBNgos6EnTrXrzzDXEZoQNFcz2kFHjgH2Lcubtm");

#[program]
pub mod construction_transparency {
    use super::*;
    pub fn create_project(
        ctx: Context<CreateProject>,
        name: String,
        location: String,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        project.authority = ctx.accounts.authority.key();
        project.name = name;
        project.location = location;
        project.contractor = ctx.accounts.contractor.key();
        project.verified = false;
        project.timestamp = Clock::get()?.unix_timestamp;

        emit!(ProjectCreated {
            project: project.key(),
            authority: project.authority,
            contractor: project.contractor,
            name: project.name.clone(),
        });

        Ok(())
    }

    pub fn add_material(
        ctx: Context<ContractorUpdate>,
        name: String,
        quantity: u32,
        quality_grade: String,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;

        project.materials.push(MaterialRecord {
            name: name.clone(),
            quantity,
            quality_grade: quality_grade.clone(),
            verified_by: None,
        });

        emit!(MaterialAdded {
            project: project.key(),
            contractor: ctx.accounts.contractor.key(),
            name,
            quantity,
            quality_grade,
        });

        Ok(())
    }

    pub fn verify_material(ctx: Context<AuthorityUpdate>, material_index: u32) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let verifier = ctx.accounts.authority.key();

        let material = project
            .materials
            .get_mut(material_index as usize)
            .ok_or(ErrorCode::InvalidMaterialIndex)?;

        require!(material.verified_by.is_none(), ErrorCode::MaterialAlreadyVerified);

        material.verified_by = Some(verifier);

        emit!(MaterialVerified {
            project: project.key(),
            verifier,
            material_index,
        });

        Ok(())
    }

    pub fn add_proof(ctx: Context<AuthorityUpdate>, proof_hash: String) -> Result<()> {
        let project = &mut ctx.accounts.project;
        project.doc_hashes.push(proof_hash.clone());

        emit!(ProofAdded {
            project: project.key(),
            authority: ctx.accounts.authority.key(),
            proof_hash,
        });

        Ok(())
    }

    pub fn final_verify(ctx: Context<AuthorityUpdate>) -> Result<()> {
        let project = &mut ctx.accounts.project;
        require!(!project.verified, ErrorCode::ProjectAlreadyVerified);

        project.verified = true;

        emit!(ProjectVerified {
            project: project.key(),
            authority: ctx.accounts.authority.key(),
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 1024,
        seeds = [b"project", authority.key().as_ref()],
        bump
    )]
    pub project: Account<'info, ConstructionProject>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: This is just a pubkey stored in the project data. We do not read or write to this account.
    pub contractor: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AuthorityUpdate<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"project", authority.key().as_ref()],
        bump
    )]
    pub project: Account<'info, ConstructionProject>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ContractorUpdate<'info> {
    #[account(
        mut,
        has_one = contractor,
        seeds = [b"project", project.authority.as_ref()],
        bump
    )]
    pub project: Account<'info, ConstructionProject>,
    pub contractor: Signer<'info>,
}

#[account]
pub struct ConstructionProject {
    pub authority: Pubkey,
    pub name: String,
    pub location: String,
    pub contractor: Pubkey,
    pub materials: Vec<MaterialRecord>,
    pub doc_hashes: Vec<String>,
    pub verified: bool,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct MaterialRecord {
    #[max_len(50)]
    pub name: String,
    pub quantity: u32,
    #[max_len(20)]
    pub quality_grade: String,
    pub verified_by: Option<Pubkey>,
}

#[event]
pub struct ProjectCreated {
    pub project: Pubkey,
    pub authority: Pubkey,
    pub contractor: Pubkey,
    pub name: String,
}

#[event]
pub struct MaterialAdded {
    pub project: Pubkey,
    pub contractor: Pubkey,
    pub name: String,
    pub quantity: u32,
    pub quality_grade: String,
}

#[event]
pub struct MaterialVerified {
    pub project: Pubkey,
    pub verifier: Pubkey,
    pub material_index: u32,
}

#[event]
pub struct ProofAdded {
    pub project: Pubkey,
    pub authority: Pubkey,
    pub proof_hash: String,
}

#[event]
pub struct ProjectVerified {
    pub project: Pubkey,
    pub authority: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Invalid material index provided.")]
    InvalidMaterialIndex,
    #[msg("The project has already been marked as verified.")]
    ProjectAlreadyVerified,
    #[msg("The material has already been verified.")]
    MaterialAlreadyVerified,
}
