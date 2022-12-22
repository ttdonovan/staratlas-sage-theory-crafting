use yew::prelude::*;

use sage_core::PrivilegePoint;

use crate::state::State;

pub enum Msg {
    AddPoint(PrivilegePoint),
    ResetAll,
}

pub struct App {
    state: State,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            state: State::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddPoint(point) => {
                self.state.counter += 1;

                self.state.add_privilege_point(point);
            }
            Msg::ResetAll => {
                self.state = State::default();
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let disabled = self.state.disabled_all();

        html! {
            <div>
                <ul>
                    <li>
                        { "Concurrent Fleet" }
                        <button disabled={disabled || self.state.disabled_concurent_fleet()}
                            onclick={ctx.link().callback(|_| Msg::AddPoint(PrivilegePoint::ConcurrentFleets))}>
                            { "+1 Fleet" }
                        </button>
                    </li>
                    <li>
                        { "Fleet Size" }
                        <button disabled={disabled || self.state.disabled_fleet_size()}
                            onclick={ctx.link().callback(|_| Msg::AddPoint(PrivilegePoint::FleetSize))}>
                            { "+1 Fleet Size" }
                        </button>
                    </li>
                    <li>
                        { "Crafting Capacity" }
                        <button disabled={disabled}
                            onclick={ctx.link().callback(|_| Msg::AddPoint(PrivilegePoint::CraftingCapacity))}>
                            { "+1 Crew (for crafting)" }
                        </button>
                    </li>
                    <li>
                        { "Starpath Pass" }
                        <button disabled={disabled || self.state.disabled_starpass_path()}
                            onclick={ctx.link().callback(|_| Msg::AddPoint(PrivilegePoint::StarpathPass))}>
                            { "+0.5%" }
                        </button>
                    </li>
                    <li>
                        { "Expedited Rescue" }
                        <button disabled={disabled || self.state.disabled_expedited_rescue()}
                            onclick={ctx.link().callback(|_| Msg::AddPoint(PrivilegePoint::ExpeditedRescue))}>
                            { "+0.5%" }
                        </button>
                    </li>
                </ul>
                <p>
                    <a onclick={ctx.link().callback(|_| Msg::ResetAll)}>{ "reset" }</a>
                </p>
                <p>{ format!("{:?}", self.state.council_rank_privileges()) }</p>
                <p>{ format!("{:?}", self.state.council_rank_privilege_points()) }</p>
                <p>{ format!("{:?}", self.state.privilege_points_counter()) }</p>
            </div>
        }
    }
}
