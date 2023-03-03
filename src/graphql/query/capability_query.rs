use diesel::{RunQueryDsl};
use crate::schema::*;

use async_graphql::*;

use crate::models::{Person, User, TeamOwnership,
    Team, Organization, Role, OrgTier, Capability, Skill, CapabilityCount, SkillDomain, CapabilityLevel};
use uuid::Uuid;

use crate::graphql::{get_connection_from_context};
use crate::common_utils::{RoleGuard, is_admin, UserRole};

#[derive(Default)]
pub struct CapabilityQuery;

#[Object]
impl CapabilityQuery {

    // Capabilities

    pub async fn capabilities(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Capability>> {

        Capability::get_all()
    }

    pub async fn capability_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Capability> {

        Capability::get_by_id(&id)
    }

    pub async fn capabilities_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Capability>> {

        Capability::get_by_name(&name)
    }

    pub async fn capabilities_by_name_and_level(
        &self, 
        _context: &Context<'_>,
        name: String,
        level: CapabilityLevel,
    ) -> Result<Vec<Capability>> {

        Capability::get_by_name_and_level(&name, level)
    }
       

    /// Return a count of the number of people who have a capability at each level of the capability
    pub async fn capability_counts_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<CapabilityCount>> {

        Capability::get_level_counts_by_name(name)
    }

    pub async fn capability_counts_by_domain(
        &self, 
        _context: &Context<'_>,
        domain: SkillDomain,
    ) -> Result<Vec<CapabilityCount>> {

        Capability::get_level_counts_by_domain(domain)
    }

    // Skills

    pub async fn skills(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Skill>> {

        Skill::get_all()
    }

    pub async fn get_skill_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Skill> {

        Skill::get_by_id(&id)
    }

    pub async fn skill_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Skill>> {

        Skill::get_by_name(name)
    }
}