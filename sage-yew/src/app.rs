use yew::html::Scope;
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
            <div class="container-fluid">
                <div class="row">
                    <h1>{ "Star Atlas: SAGE Council Rank Theory Crafting" }</h1>
                </div>
                <div class="row">
                    <div class="col">
                        <h2>{ "Council Rank Privileges" }</h2>
                        <table class="table table-sm table-hover">
                            <caption>{ format!("Council Rank Privileges and total points: {}", self.state.privilege_points_counter()) }</caption>
                            <thead>
                                <th scope="col">{ "Privilege" }</th>
                                <th scope="col">{ "Assigned Points" }</th>
                            </thead>
                            <tbody>
                                <tr>
                                    <th scope="row">{ "Concurrent Fleets" }</th>
                                    <td>{ self.state.council_rank_privileges().concurrent_fleets }</td>
                                </tr>
                                <tr>
                                    <th scope="row">{ "Fleet Size" }</th>
                                    <td>{ self.state.council_rank_privileges().fleet_size }</td>
                                </tr>
                                <tr>
                                    <th scope="row">{ "Crafing Capacity" }</th>
                                    <td>{ self.state.council_rank_privileges().crafting_capacity }</td>
                                </tr>
                                <tr>
                                    <th scope="row">{ "Starpath Pass" }</th>
                                    <td>{ format!("{}%", self.state.council_rank_privileges().starpath_pass) }</td>
                                </tr>
                                <tr>
                                    <th scope="row">{ "Expedited Rescue" }</th>
                                    <td>{ format!("{}%", self.state.council_rank_privileges().expedited_rescue) }</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
                <div class="row">
                    <div class="col">
                        <div class="btn-toolbar" role="toolbar">
                            <div class="btn-group-vertical" role="group">
                                <button type="button"
                                    class="btn btn-outline-primary"
                                    disabled={disabled || self.state.disabled_concurent_fleet()}
                                    onclick={ctx.link().callback(|_| Msg::AddPoint(PrivilegePoint::ConcurrentFleets))}>
                                    { "Concurrent Fleets (+1 Fleet)" }
                                </button>
                                <button type="button"
                                    class="btn btn-outline-primary"
                                    disabled={disabled || self.state.disabled_fleet_size()}
                                    onclick={ctx.link().callback(|_| Msg::AddPoint(PrivilegePoint::FleetSize))}>
                                    { "Fleet Size (+1 Fleet Size)" }
                                </button>
                                <button type="button"
                                    class="btn btn-outline-primary"
                                    disabled={disabled}
                                    onclick={ctx.link().callback(|_| Msg::AddPoint(PrivilegePoint::CraftingCapacity))}>
                                    { "Crafing Capacity (+1 Crew for crafting)" }
                                </button>
                                <button type="button"
                                    class="btn btn-outline-primary"
                                    disabled={disabled || self.state.disabled_starpass_path()}
                                    onclick={ctx.link().callback(|_| Msg::AddPoint(PrivilegePoint::StarpathPass))}>
                                    { "Starpath Pass (+0.5%)" }
                                </button>
                                <button type="button"
                                    class="btn btn-outline-primary"
                                    disabled={disabled || self.state.disabled_expedited_rescue()}
                                    onclick={ctx.link().callback(|_| Msg::AddPoint(PrivilegePoint::ExpeditedRescue))}>
                                    { "Expedited Rescue (+0.5%)" }
                                </button>
                                <button type="button"
                                    class="btn btn-outline-secondary"
                                    onclick={ctx.link().callback(|_| Msg::ResetAll)}>
                                    { "Reset All"}
                                </button>
                            </div>
                        </div>
                    </div>
                    <div class="col">
                        { self.view_base64_url_safe_input(ctx.link()) }
                    </div>
                </div>
                <div class="row">
                    <div class="col">
                        <h3>{ "Privilege Points" }</h3>
                        <ol class="list-group list-group-numbered">
                            { for self.state.council_rank_privilege_points().iter().enumerate().map(|e| self.view_privilege_point_list_group_item(e)) }
                        </ol>
                    </div>
                    <div class="col">
                        <h3>{ "Base64 URL Safe" }</h3>
                        <p class="font-monospace text-break">{ self.state.council_rank_base64_url_safe() }</p>
                    </div>
                </div>
            </div>
        }
    }
}

impl App {
    fn view_base64_url_safe_input(&self, _link: &Scope<Self>) -> Html {
        html! {
            <div class="mb-4">
                <textarea
                    class="form-control"
                    placeholder="(WIP) Paste a Base64 URL Safe string to load a Council Rank build."
                    rows="8"
                >
                </textarea>
            </div>
        }
    }

    fn view_privilege_point_list_group_item(
        &self,
        (_idx, privilege_point): (usize, &PrivilegePoint),
    ) -> Html {
        html! {
            <li class="list-group-item">{ format!("{:?}", privilege_point) }</li>
        }
    }
}
