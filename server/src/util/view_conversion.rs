use crate::{
    models::{
        budget_planner::{BudgetPlanner, PersonalBudget},
        documents::Document,
        expenses::Expense,
        itinerary::ItineraryItem,
        location::Location,
        trip::{Trip, TripData},
        user::{Collaborator, User},
    },
    views::{
        EncodableBudgetPlan, EncodableCollaborator, EncodableDocument, EncodableExpense,
        EncodableGroupBudget, EncodableItineraryItem, EncodableLocation, EncodablePersonalBudget,
        EncodableTripData, EncodableTripOverview, EncodableUser, EncodableUserPreview,
        ItineraryExpense,
    },
};

impl From<&Collaborator> for EncodableCollaborator {
    fn from(value: &Collaborator) -> Self {
        Self {
            id: value.id,
            username: value.username.clone(),
            avatar: value.avatar.clone(),
            permission: value.permission.clone(),
        }
    }
}

impl From<User> for EncodableUser {
    fn from(value: User) -> Self {
        EncodableUser {
            id: value.id,
            username: value.username,
            email: value.email,
            avatar: value.avatar,
        }
    }
}

impl From<Trip> for EncodableTripOverview {
    fn from(value: Trip) -> Self {
        Self {
            id: value.id,
            title: value.title,
            banner_image: value.banner_image,
            start_date: value.start_date,
            end_date: value.end_date,
            no_collaborators: value.no_collaborators,
        }
    }
}

impl From<User> for EncodableUserPreview {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            avatar: value.avatar,
        }
    }
}

impl From<&(Expense, Vec<User>)> for EncodableExpense {
    fn from(value: &(Expense, Vec<User>)) -> Self {
        let expense = value.0.clone();
        let payers = &value.1;

        Self {
            id: expense.id,
            title: expense.title,
            cost: expense.cost,
            currency: expense.currency,
            payers: payers
                .iter()
                .map(|p| EncodableUserPreview::from(p.clone()))
                .collect(),
        }
    }
}

impl From<Document> for EncodableDocument {
    fn from(value: Document) -> Self {
        Self {
            id: value.id,
            filename: value.filename,
            size_bytes: value.file_size,
        }
    }
}

impl From<PersonalBudget> for EncodablePersonalBudget {
    fn from(value: PersonalBudget) -> Self {
        Self {
            currency: value.currency,
            total_budget: value.total_budget,
            accommodation_budget: value.accommodation_budget,
            transportation_budget: value.transportation_budget,
            food_dining_budget: value.food_dining_budget,
            activities_budget: value.activities_budget,
            shopping_budget: value.shopping_budget,
        }
    }
}

impl From<BudgetPlanner> for EncodableGroupBudget {
    fn from(value: BudgetPlanner) -> Self {
        Self {
            currency: value.currency,
            total_budget: value.total_budget,
            accommodation_budget: value.accommodation_budget,
            transportation_budget: value.transportation_budget,
            food_dining_budget: value.food_dining_budget,
            activities_budget: value.activities_budget,
            shopping_budget: value.shopping_budget,
        }
    }
}

impl From<Location> for EncodableLocation {
    fn from(value: Location) -> Self {
        Self {
            display_name: value.display_name,
            address: value.address,
            longitude: value.longitude,
            latitude: value.latitude,
        }
    }
}

impl From<Expense> for ItineraryExpense {
    fn from(value: Expense) -> Self {
        Self {
            cost: value.cost,
            currency: value.currency,
        }
    }
}

impl From<&(ItineraryItem, Option<Location>, Option<Expense>)> for EncodableItineraryItem {
    fn from(value: &(ItineraryItem, Option<Location>, Option<Expense>)) -> Self {
        let itinerary_item = value.0.clone();
        let location = value.1.clone();
        let expense = value.2.clone();

        Self {
            id: Some(itinerary_item.id),
            activity_type: itinerary_item.activity_type,
            title: itinerary_item.title,
            location: if let Some(location) = location {
                Some(EncodableLocation::from(location))
            } else {
                None
            },
            start_time: itinerary_item.start_time,
            end_time: itinerary_item.end_time,
            cost: if let Some(expense) = expense {
                Some(ItineraryExpense::from(expense))
            } else {
                None
            },
            notes: itinerary_item.notes,
        }
    }
}

impl From<TripData> for EncodableTripData {
    fn from(value: TripData) -> Self {
        let collaborators: Vec<EncodableCollaborator> = value
            .collaborators
            .iter()
            .map(|c| EncodableCollaborator::from(c))
            .collect();

        let use_personal_budget = value.personal_budget_plan.personal_budget_enabled.clone();

        let budget_plan = EncodableBudgetPlan {
            group_budget: EncodableGroupBudget::from(value.budget_plan),
            personal_budget: EncodablePersonalBudget::from(value.personal_budget_plan),
            expenses: value
                .trip_expenses
                .iter()
                .map(|e| EncodableExpense::from(e))
                .collect(),
            use_personal_budget,
        };

        let itinerary: Vec<EncodableItineraryItem> = value
            .itinerary_items
            .iter()
            .map(|items| EncodableItineraryItem::from(items))
            .collect();

        let documents: Vec<EncodableDocument> = value
            .documents
            .iter()
            .map(|d| EncodableDocument::from(d.clone()))
            .collect();

        Self {
            id: value.trip.id,
            title: value.trip.title,
            banner_image: value.trip.banner_image,
            start_date: value.trip.start_date,
            end_date: value.trip.end_date,
            collaborators,
            budget_plan,
            itinerary,
            documents,
        }
    }
}
