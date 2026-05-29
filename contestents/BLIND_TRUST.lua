-- name: BLIND TRUST
-- cooperates for first 20 rounds no matter what, then becomes responsive

function decide(round, history)
    if round <= 20 then
        return "cooperate"
    else
        -- After 20 rounds, use Tit-for-Tat
        return history[#history]
    end
end
