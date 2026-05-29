-- name: APPEASER
-- tries to appease by cooperating after any defection

function decide(round, history)
    if round == 1 then return "cooperate" end
    if history[#history] == "defect" then
        return "cooperate"
    else
        return "cooperate"
    end
end
