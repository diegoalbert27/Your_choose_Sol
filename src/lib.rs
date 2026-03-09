use anchor_lang::prelude::*;

declare_id!("BEkiTv1LxUXrfZtQD61Di9MNMzPqRaqBHwcKkdkRrB6A");

#[program]
pub mod your_choose {
    use super::*;

    pub fn create_topic(context: Context<NewTopic>, topic_name: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let candidates: Vec<Candidate> = Vec::new();
        let participants: Vec<Pubkey> = Vec::new();

        context.accounts.topic.set_inner(Topic {
            owner: owner_id,
            topic_name,
            candidates,
            participants,
        });

        Ok(())
    }

    pub fn add_candidate(context: Context<NewCandidate>, name: String) -> Result<()> {
        require!(
            context.accounts.topic.owner == context.accounts.owner.key(),
            Errors::YouAreNotOwner
        );

        let candidate = Candidate {
            name,
            votes: 0,
            is_active: true,
        };

        context.accounts.topic.candidates.push(candidate);

        Ok(())
    }

    pub fn get_canditates(context: Context<GetCandidates>) -> Result<()> {
        require!(
            context.accounts.topic.owner == context.accounts.owner.key(),
            Errors::YouAreNotOwner
        );

        msg!(
            "La lista de candidatos actualmente es: {:#?}",
            context.accounts.topic.candidates
        );
        Ok(())
    }

    pub fn get_participants(context: Context<GetParticipants>) -> Result<()> {
        require!(
            context.accounts.topic.owner == context.accounts.owner.key(),
            Errors::YouAreNotOwner
        );

        msg!(
            "La lista de participantes actualmente es: {:#?}",
            context.accounts.topic.participants
        );
        Ok(())
    }

    pub fn update_candidate_state(context: Context<NewCandidate>, name: String) -> Result<()> {
        require!(
            context.accounts.topic.owner == context.accounts.owner.key(),
            Errors::YouAreNotOwner
        );

        let candidates = &mut context.accounts.topic.candidates;
        for index in 0..candidates.len() {
            let state = candidates[index].is_active;

            if candidates[index].name == name {
                let new_is_active = !state;
                candidates[index].is_active = new_is_active;
                msg!(
                    "El candidato: {} tiene un nuevo status: {}",
                    name,
                    new_is_active
                );

                return Ok(());
            }
        }

        Err(Errors::CandidateWasNotFind.into())
    }

    pub fn delete_candidate(context: Context<NewCandidate>, name: String) -> Result<()> {
        require!(
            context.accounts.topic.owner == context.accounts.owner.key(),
            Errors::YouAreNotOwner
        );

        let candidates = &mut context.accounts.topic.candidates;

        for index in 0..candidates.len() {
            if candidates[index].name == name {
                candidates.remove(index);
                msg!("Candidate {} removed!", name);
                return Ok(());
            }
        }
        Err(Errors::CandidateWasNotFind.into())
    }

    pub fn add_vote_to_candidate(context: Context<NewCandidate>, name: String) -> Result<()> {
        let topic = &mut context.accounts.topic;
        let voter_key = context.accounts.owner.key();

        let already_voted = topic.participants.iter().any(|&p| p == voter_key);
        
        require!(!already_voted, Errors::ErrorInParticipant); // Error si ya existe

        let mut candidate_found = false;
        for candidate in topic.candidates.iter_mut() {
            if candidate.name == name {
                candidate.votes += 1;
                candidate_found = true;
                msg!("El candidato: {} tiene un nuevo voto", name);
                break;
            }
        }

        if !candidate_found {
            return Err(Errors::CandidateWasNotFind.into());
        }

        topic.participants.push(voter_key);

        Ok(())
    }
}

#[error_code]
pub enum Errors {
    #[msg("Error, no eres el propietario del topico que deseas modificar")]
    YouAreNotOwner,

    #[msg("Error, el candidato que buscas no existe")]
    CandidateWasNotFind,

    #[msg("Error, Tu participacion ya ha sido tomada en cuenta")]
    ErrorInParticipant,
}

#[account]
#[derive(InitSpace)]
pub struct Topic {
    owner: Pubkey,

    #[max_len(60)]
    topic_name: String,

    #[max_len(10)]
    candidates: Vec<Candidate>,

    #[max_len(100)]
    participants: Vec<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Candidate {
    #[max_len(60)]
    name: String,

    votes: u32,

    is_active: bool,
}

#[derive(Accounts)]
pub struct NewTopic<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Topic::INIT_SPACE + 8,
        seeds = [b"topic", owner.key().as_ref()],
        bump
    )]
    pub topic: Account<'info, Topic>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NewCandidate<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub topic: Account<'info, Topic>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetParticipants<'info> {
    pub topic: Account<'info, Topic>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetCandidates<'info> {
    pub topic: Account<'info, Topic>,
    pub owner: Signer<'info>,
}
